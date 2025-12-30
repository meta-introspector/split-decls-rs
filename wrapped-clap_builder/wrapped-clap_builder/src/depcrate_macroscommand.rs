// Generated macro for command (macro)
macro_rules! Depcrate_macroscommand {
() => {
// Module: crate::macros
// Provides: {"command"}
// Dependencies: {}
# [doc = " Requires `cargo` feature flag to be enabled."] # [cfg (not (feature = "cargo"))] # [macro_export] macro_rules ! command { () => { { compile_error ! ("`cargo` feature flag is required") ; } } ; ($ name : expr) => { { compile_error ! ("`cargo` feature flag is required") ; } } ; }
};
}
