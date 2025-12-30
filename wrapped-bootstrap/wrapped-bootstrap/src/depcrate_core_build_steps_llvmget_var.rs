// Generated macro for get_var (function)
macro_rules! Depcrate_core_build_steps_llvmget_var {
() => {
// Module: crate::core::build_steps::llvm
// Provides: {"get_var"}
// Dependencies: {}
fn get_var (var_base : & str , host : & str , target : & str) -> Option < OsString > { let kind = if host == target { "HOST" } else { "TARGET" } ; let target_u = target . replace ('-' , "_") ; env :: var_os (format ! ("{var_base}_{target}")) . or_else (| | env :: var_os (format ! ("{var_base}_{target_u}"))) . or_else (| | env :: var_os (format ! ("{kind}_{var_base}"))) . or_else (| | env :: var_os (var_base)) }
};
}
