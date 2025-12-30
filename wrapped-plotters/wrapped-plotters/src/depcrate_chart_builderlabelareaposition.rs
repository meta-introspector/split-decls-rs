// Generated macro for LabelAreaPosition (enum)
macro_rules! Depcrate_chart_builderLabelAreaPosition {
() => {
// Module: crate::chart::builder
// Provides: {"LabelAreaPosition"}
// Dependencies: {}
# [doc = "\nSpecifies one of the four label positions around the figure.\n\nThis is used to configure the label area size with function\n[`ChartBuilder::set_label_area_size()`].\n\n# Example\n\n```\nuse plotters::prelude::*;\nlet drawing_area = SVGBackend::new(\"label_area_position.svg\", (300, 200)).into_drawing_area();\ndrawing_area.fill(&WHITE).unwrap();\nlet mut chart_builder = ChartBuilder::on(&drawing_area);\nchart_builder.set_label_area_size(LabelAreaPosition::Bottom, 60).set_label_area_size(LabelAreaPosition::Left, 35);\nlet mut chart_context = chart_builder.build_cartesian_2d(0.0..4.0, 0.0..3.0).unwrap();\nchart_context.configure_mesh().x_desc(\"Spacious X label area\").y_desc(\"Narrow Y label area\").draw().unwrap();\n```\n\nThe result is a chart with a spacious X label area and a narrow Y label area:\n\n![](https://cdn.jsdelivr.net/gh/facorread/plotters-doc-data@9ca6541/apidoc/label_area_position.svg)\n\n# See also\n\n[`ChartBuilder::set_left_and_bottom_label_area_size()`]\n"] # [derive (Copy , Clone)] pub enum LabelAreaPosition { # [doc = " Top of the figure"] Top = 0 , # [doc = " Bottom of the figure"] Bottom = 1 , # [doc = " Left side of the figure"] Left = 2 , # [doc = " Right side of the figure"] Right = 3 , }
};
}
