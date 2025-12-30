// Generated macro for allow_by_no_color_spec (function)
macro_rules! Depcrate_colorallow_by_no_color_spec {
() => {
// Module: crate::color
// Provides: {"allow_by_no_color_spec"}
// Dependencies: {}
fn allow_by_no_color_spec () -> bool { std :: env :: var_os ("NO_COLOR") . is_none () }
};
}
