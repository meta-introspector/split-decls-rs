// Generated macro for inject_rustc_tool_env (function)
macro_rules! Depcrate_envinject_rustc_tool_env {
() => {
// Module: crate::env
// Provides: {"inject_rustc_tool_env"}
// Dependencies: {}
pub (crate) fn inject_rustc_tool_env (env : & mut Env , cargo_name : & str , kind : TargetKind) { _ = kind ; env . set ("CARGO_CRATE_NAME" , cargo_name . replace ('-' , "_")) ; }
};
}
