// Generated macro for ext_err_if (macro)
macro_rules! Depcrate_match_tokenext_err_if {
() => {
// Module: crate::match_token
// Provides: {"ext_err_if"}
// Dependencies: {}
macro_rules ! ext_err_if { ($ condition : expr , $ span : expr , $ message : expr) => { if $ condition { return Err (($ span , $ message)) } } }
};
}
