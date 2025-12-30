// Generated macro for allow_clicolors_spec (function)
macro_rules! Depcrate_colorallow_clicolors_spec {
() => {
// Module: crate::color
// Provides: {"allow_clicolors_spec"}
// Dependencies: {}
fn allow_clicolors_spec () -> bool { evar_equals (evar_with_default ("CLICOLOR" , "1") , "1") || evar_not_equals (evar_with_default ("CLICOLOR_FORCE" , "0") , "0") }
};
}
