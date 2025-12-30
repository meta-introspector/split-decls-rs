// Generated macro for impl_815 (impl)
macro_rules! Depcrate_style_colorimpl_815 {
() => {
// Module: crate::style::color
// Provides: {"impl_815"}
// Dependencies: {}
impl < P : Palette > Color for PaletteColor < P > { # [inline (always)] fn to_backend_color (& self) -> BackendColor { BackendColor { rgb : P :: COLORS [self . 0] , alpha : 1.0 , } } }
};
}
