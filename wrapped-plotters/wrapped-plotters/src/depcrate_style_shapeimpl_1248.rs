// Generated macro for impl_1248 (impl)
macro_rules! Depcrate_style_shapeimpl_1248 {
() => {
// Module: crate::style::shape
// Provides: {"impl_1248"}
// Dependencies: {}
impl BackendStyle for ShapeStyle { # [doc = " Returns the color as interpreted by the backend."] fn color (& self) -> BackendColor { self . color . to_backend_color () } # [doc = " Returns the stroke width."] fn stroke_width (& self) -> u32 { self . stroke_width } }
};
}
