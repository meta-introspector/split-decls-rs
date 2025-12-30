// Generated macro for LineSeries (struct)
macro_rules! Depcrate_series_line_seriesLineSeries {
() => {
// Module: crate::series::line_series
// Provides: {"LineSeries"}
// Dependencies: {}
# [doc = "\nThe line series object, which takes an iterator of data points in guest coordinate system\nand creates appropriate lines and points with the given style.\n\n# Example\n\n```\nuse plotters::prelude::*;\nlet x_values = [0.0f64, 1., 2., 3., 4.];\nlet drawing_area = SVGBackend::new(\"line_series_point_size.svg\", (300, 200)).into_drawing_area();\ndrawing_area.fill(&WHITE).unwrap();\nlet mut chart_builder = ChartBuilder::on(&drawing_area);\nchart_builder.margin(10).set_left_and_bottom_label_area_size(20);\nlet mut chart_context = chart_builder.build_cartesian_2d(0.0..4.0, 0.0..3.0).unwrap();\nchart_context.configure_mesh().draw().unwrap();\nchart_context.draw_series(LineSeries::new(x_values.map(|x| (x, 0.3 * x)), BLACK)).unwrap();\nchart_context.draw_series(LineSeries::new(x_values.map(|x| (x, 2.5 - 0.05 * x * x)), RED)\n    .point_size(5)).unwrap();\nchart_context.draw_series(LineSeries::new(x_values.map(|x| (x, 2. - 0.1 * x * x)), BLUE.filled())\n    .point_size(4)).unwrap();\n```\n\nThe result is a chart with three line series; two of them have their data points highlighted:\n\n![](https://cdn.jsdelivr.net/gh/facorread/plotters-doc-data@64e0a28/apidoc/line_series_point_size.svg)\n"] pub struct LineSeries < DB : DrawingBackend , Coord > { style : ShapeStyle , data : Vec < Coord > , point_idx : usize , point_size : u32 , phantom : PhantomData < DB > , }
};
}
