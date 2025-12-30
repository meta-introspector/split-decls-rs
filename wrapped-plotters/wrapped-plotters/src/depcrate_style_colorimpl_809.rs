// Generated macro for impl_809 (impl)
macro_rules! Depcrate_style_colorimpl_809 {
() => {
// Module: crate::style::color
// Provides: {"impl_809"}
// Dependencies: {}
impl < T : Color > Color for & '_ T { fn to_backend_color (& self) -> BackendColor { < T as Color > :: to_backend_color (* self) } }
};
}
