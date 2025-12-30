// Generated macro for Cubiod (struct)
macro_rules! Depcrate_element_basic_shapes_3dCubiod {
() => {
// Module: crate::element::basic_shapes_3d
// Provides: {"Cubiod"}
// Dependencies: {}
# [doc = "\nRepresents a cuboid, a six-faced solid.\n\n# Examples\n\n```\nuse plotters::prelude::*;\nlet drawing_area = SVGBackend::new(\"cuboid.svg\", (300, 200)).into_drawing_area();\ndrawing_area.fill(&WHITE).unwrap();\nlet mut chart_builder = ChartBuilder::on(&drawing_area);\nlet mut chart_context = chart_builder.margin(20).build_cartesian_3d(0.0..3.5, 0.0..2.5, 0.0..1.5).unwrap();\nchart_context.configure_axes().x_labels(4).y_labels(3).z_labels(2).draw().unwrap();\nlet cubiod = Cubiod::new([(0.,0.,0.), (3.,2.,1.)], BLUE.mix(0.2), BLUE);\nchart_context.draw_series(std::iter::once(cubiod)).unwrap();\n```\n\nThe result is a semi-transparent cuboid with blue edges:\n\n![](https://cdn.jsdelivr.net/gh/facorread/plotters-doc-data@b6703f7/apidoc/cuboid.svg)\n"] pub struct Cubiod < X , Y , Z > { face_style : ShapeStyle , edge_style : ShapeStyle , vert : [(X , Y , Z) ; 8] , }
};
}
