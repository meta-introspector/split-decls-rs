// Generated macro for EmptyElement (struct)
macro_rules! Depcrate_element_composableEmptyElement {
() => {
// Module: crate::element::composable
// Provides: {"EmptyElement"}
// Dependencies: {}
# [doc = "\nAn empty composable element. This is the starting point of a composed element.\n\n# Example\n\n```\nuse plotters::prelude::*;\nlet data = [(1.0, 3.3), (2., 2.1), (3., 1.5), (4., 1.9), (5., 1.0)];\nlet drawing_area = SVGBackend::new(\"composable.svg\", (300, 200)).into_drawing_area();\ndrawing_area.fill(&WHITE).unwrap();\nlet mut chart_builder = ChartBuilder::on(&drawing_area);\nchart_builder.margin(7).set_left_and_bottom_label_area_size(20);\nlet mut chart_context = chart_builder.build_cartesian_2d(0.0..5.5, 0.0..5.5).unwrap();\nchart_context.configure_mesh().draw().unwrap();\nchart_context.draw_series(data.map(|(x, y)| {\n    EmptyElement::at((x, y)) // Use the guest coordinate system with EmptyElement\n    + Circle::new((0, 0), 10, BLUE) // Use backend coordinates with the rest\n    + Cross::new((4, 4), 3, RED)\n    + Pixel::new((4, -4), RED)\n    + TriangleMarker::new((-4, -4), 4, RED)\n})).unwrap();\n```\n\nThe result is a data series where each point consists of a circle, a cross, a pixel, and a triangle:\n\n![](https://cdn.jsdelivr.net/gh/facorread/plotters-doc-data@06d370f/apidoc/composable.svg)\n\n"] pub struct EmptyElement < Coord , DB : DrawingBackend > { coord : Coord , phantom : PhantomData < DB > , }
};
}
