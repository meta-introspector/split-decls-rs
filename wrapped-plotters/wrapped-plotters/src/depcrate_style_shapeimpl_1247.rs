// Generated macro for impl_1247 (impl)
macro_rules! Depcrate_style_shapeimpl_1247 {
() => {
// Module: crate::style::shape
// Provides: {"impl_1247"}
// Dependencies: {}
impl < T : Color > From < T > for ShapeStyle { fn from (f : T) -> Self { ShapeStyle { color : f . to_rgba () , filled : false , stroke_width : 1 , } } }
};
}
