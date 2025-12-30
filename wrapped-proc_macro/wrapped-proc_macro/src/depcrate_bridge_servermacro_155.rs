// Generated macro for macro_155 (macro)
macro_rules! Depcrate_bridge_servermacro_155 {
() => {
// Module: crate::bridge::server
// Provides: {"macro_155"}
// Dependencies: {}
thread_local ! { # [doc = " While running a proc-macro with the same-thread executor, this flag will"] # [doc = " be set, forcing nested proc-macro invocations (e.g. due to"] # [doc = " `TokenStream::expand_expr`) to be run using a cross-thread executor."] # [doc = ""] # [doc = " This is required as the thread-local state in the proc_macro client does"] # [doc = " not handle being re-entered, and will invalidate all `Symbol`s when"] # [doc = " entering a nested macro."] static ALREADY_RUNNING_SAME_THREAD : Cell < bool > = const { Cell :: new (false) } ; }
};
}
