// Generated macro for AreaSeries (struct)
macro_rules! Depcrate_series_area_seriesAreaSeries {
() => {
// Module: crate::series::area_series
// Provides: {"AreaSeries"}
// Dependencies: {}
# [doc = "\nAn area series is similar to a line series but uses a filled polygon.\nIt takes an iterator of data points in guest coordinate system\nand creates appropriate lines and points with the given style.\n\n# Example\n\n```\nuse plotters::prelude::*;\nlet x_values = [0.0f64, 1., 2., 3., 4.];\nlet drawing_area = SVGBackend::new(\"area_series.svg\", (300, 200)).into_drawing_area();\ndrawing_area.fill(&WHITE).unwrap();\nlet mut chart_builder = ChartBuilder::on(&drawing_area);\nchart_builder.margin(10).set_left_and_bottom_label_area_size(20);\nlet mut chart_context = chart_builder.build_cartesian_2d(0.0..4.0, 0.0..3.0).unwrap();\nchart_context.configure_mesh().draw().unwrap();\nchart_context.draw_series(AreaSeries::new(x_values.map(|x| (x, 0.3 * x)), 0., BLACK.mix(0.2))).unwrap();\nchart_context.draw_series(AreaSeries::new(x_values.map(|x| (x, 2.5 - 0.05 * x * x)), 0., RED.mix(0.2))).unwrap();\nchart_context.draw_series(AreaSeries::new(x_values.map(|x| (x, 2. - 0.1 * x * x)), 0., BLUE.mix(0.2)).border_style(BLUE)).unwrap();\n```\n\nThe result is a chart with three line series; one of them has a highlighted blue border:\n\n![](https://cdn.jsdelivr.net/gh/facorread/plotters-doc-data@b6703f7/apidoc/area_series.svg)\n"] pub struct AreaSeries < DB : DrawingBackend , X : Clone , Y : Clone > { area_style : ShapeStyle , border_style : ShapeStyle , baseline : Y , data : Vec < (X , Y) > , state : u32 , _p : std :: marker :: PhantomData < DB > , }
};
}
