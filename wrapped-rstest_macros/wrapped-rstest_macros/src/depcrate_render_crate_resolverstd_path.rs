// Generated macro for std_path (function)
macro_rules! Depcrate_render_crate_resolverstd_path {
() => {
// Module: crate::render::crate_resolver
// Provides: {"std_path"}
// Dependencies: {}
pub fn std_path () -> syn :: Path { let rstest = crate_name () ; parse_quote ! { # rstest :: __std } }
};
}
