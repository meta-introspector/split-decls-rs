// Generated macro for impl_472 (impl)
macro_rules! Depcrate_drawing_areaimpl_472 {
() => {
// Module: crate::drawing::area
// Provides: {"impl_472"}
// Dependencies: {}
impl < DB : DrawingBackend , CT : CoordTranslate + Clone > Clone for DrawingArea < DB , CT > { fn clone (& self) -> Self { Self { backend : self . backend . clone () , rect : self . rect . clone () , coord : self . coord . clone () , } } }
};
}
