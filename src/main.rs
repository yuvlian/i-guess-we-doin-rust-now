use image::{RgbImage, Rgb};
use imageproc::drawing::{draw_line_segment_mut, draw_polygon_mut, draw_filled_circle_mut, draw_hollow_circle_mut};
use imageproc::point::Point;

fn main() {
    let resolution = (498, 307);
    let mut img = RgbImage::new(resolution.0, resolution.1);

    for y in 0..resolution.1 {
        for x in 0..resolution.0 {
            img.put_pixel(x, y, Rgb([255, 255, 255]));
        }
    }

    let lines = vec![
        ((81, 282), (75, 174)),
        ((77, 172), (181, 87)),
        ((81, 282), (1, 257)),
        ((181, 86), (91, 74)),
        ((27, 68), (62, 72)),
        ((27, 68), (3, 81)),
        ((59, 0), (62, 104)),
        ((91, 102), (86, 0)),
        ((115, 284), (377, 306)),
        ((105, 186), (118, 287)),
        ((167, 163), (105, 187)),
        ((167, 163), (168, 245)),
        ((115, 238), (497, 264)),
        ((109, 153), (139, 141)),
        ((107, 177), (107, 154)),
        ((106, 177), (142, 161)),
        ((139, 139), (143, 162)),
        ((179, 87), (185, 245)),
        ((81, 24), (75, 37)),
        ((83, 39), (78, 48)),
        ((106, 161), (139, 147)),
        ((107, 170), (140, 154)),
        ((112, 150), (115, 171)),
        ((117, 148), (123, 169)),
        ((124, 144), (129, 165)),
        ((130, 143), (135, 163)),
        ((316, 175), (286, 202)),
        ((286, 205), (312, 189)),
        ((306, 255), (311, 190)),
        ((305, 256), (329, 256)),
        ((336, 231), (328, 257)),
        ((335, 232), (345, 255)),
        ((344, 255), (367, 254)), 
        ((351, 184), (365, 254)),
        ((351, 185), (371, 210)),
        ((371, 210), (377, 205)),
        ((349, 173), (375, 205)),
        ((351, 176), (344, 150)),
        ((313, 177), (323, 146)),
        ((310, 143), (350, 151)),
        ((310, 142), (318, 139)),
        ((351, 149), (358, 139)),
        ((322, 137), (358, 139)),
        ((317, 138), (326, 121)),
        ((346, 122), (349, 139)),
        ((326, 121), (332, 123)),
        ((330, 122), (333, 117)),
        ((332, 116), (338, 117)),
        ((336, 117), (340, 124)),
        ((338, 123), (346, 122))
    ];

    for &(start, end) in &lines {
        draw_line_segment_mut(&mut img, (start.0 as f32, start.1 as f32), (end.0 as f32, end.1 as f32), Rgb([0, 0, 0]));
    }

    let triangles = vec![
        vec![Point::new(141, 226), Point::new(119, 258), Point::new(153, 257)],
        vec![Point::new(205, 226), Point::new(179, 263), Point::new(225, 264)],
        vec![Point::new(351, 233), Point::new(327, 270), Point::new(373, 270)]
    ];

    for triangle in triangles {
        draw_polygon_mut(&mut img, &triangle, Rgb([237, 28, 36]));
    }

    let circle_center = (267, 253);
    let circle_radius = 18;
    draw_filled_circle_mut(&mut img, circle_center, circle_radius, Rgb([0, 162, 232]));
    draw_hollow_circle_mut(&mut img, circle_center, circle_radius, Rgb([0, 0, 0]));

    let weird_shapes = vec![
        vec![Point::new(153, 124), Point::new(149, 137), Point::new(155, 149), Point::new(161, 145), Point::new(160, 127)],
        vec![Point::new(60, 101), Point::new(47, 117), Point::new(59, 130), Point::new(75, 131), Point::new(91, 129), Point::new(103, 121), Point::new(104, 112), Point::new(105, 105), Point::new(93, 97), Point::new(78, 107)],
        vec![Point::new(160, 129), Point::new(155, 134), Point::new(156, 143), Point::new(164, 145), Point::new(170, 144), Point::new(176, 143), Point::new(181, 135), Point::new(174, 129)],
        vec![Point::new(167, 133), Point::new(171, 137), Point::new(168, 142), Point::new(161, 141), Point::new(161, 134)]
    ];

    let outline_color = Rgb([0, 0, 0]);

    for shape in weird_shapes {
        draw_polygon_mut(&mut img, &shape, Rgb([255, 255, 255]));
        for i in 0..shape.len() {
            let start = shape[i];
            let end = shape[(i + 1) % shape.len()];
            draw_line_segment_mut(&mut img, (start.x as f32, start.y as f32), (end.x as f32, end.y as f32), outline_color);
        }
    }

    let circles = vec![
        ((57, 119), 5),
        ((72, 122), 6),
        ((87, 120), 6),
        ((96, 111), 5)
    ];

    for &(center, radius) in &circles {
        draw_filled_circle_mut(&mut img, center, radius, Rgb([255, 255, 255]));
        draw_hollow_circle_mut(&mut img, center, radius, Rgb([0, 0, 0]));
    }

    img.save("circles.png").unwrap();
}
