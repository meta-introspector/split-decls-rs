// Generated macro for backtrace_if_absent (macro)
macro_rules! Depcrate_backtracebacktrace_if_absent {
() => {
// Module: crate::backtrace
// Provides: {"backtrace_if_absent"}
// Dependencies: {}
# [cfg (all (any (feature = "std" , not (anyhow_no_core_error)) , not (std_backtrace) , not (feature = "backtrace") ,))] macro_rules ! backtrace_if_absent { ($ err : expr) => { None } ; }
};
}
