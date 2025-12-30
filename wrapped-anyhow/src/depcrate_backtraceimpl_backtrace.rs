// Generated macro for impl_backtrace (macro)
macro_rules! Depcrate_backtraceimpl_backtrace {
() => {
// Module: crate::backtrace
// Provides: {"impl_backtrace"}
// Dependencies: {}
# [cfg (all (not (std_backtrace) , feature = "backtrace"))] macro_rules ! impl_backtrace { () => { impl core :: fmt :: Debug + core :: fmt :: Display } ; }
};
}
