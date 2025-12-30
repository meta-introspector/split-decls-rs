// Generated macro for impl_484 (impl)
macro_rules! Depcrate_drawing_areaimpl_484 {
() => {
// Module: crate::drawing::area
// Provides: {"impl_484"}
// Dependencies: {}
impl < DB : DrawingBackend , CT : CoordTranslate > DrawingArea < DB , CT > { # [doc = " Returns the coordinates by value"] pub fn into_coord_spec (self) -> CT { self . coord } # [doc = " Returns the coordinates by reference"] pub fn as_coord_spec (& self) -> & CT { & self . coord } # [doc = " Returns the coordinates by mutable reference"] pub fn as_coord_spec_mut (& mut self) -> & mut CT { & mut self . coord } }
};
}
