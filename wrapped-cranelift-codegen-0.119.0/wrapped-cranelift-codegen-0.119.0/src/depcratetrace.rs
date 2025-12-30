// Generated macro for trace (macro)
macro_rules! Depcratetrace {
() => {
// Module: crate
// Provides: {"trace"}
// Dependencies: {}
# [doc = " Even when trace logging is disabled, the trace macro has a significant performance cost so we"] # [doc = " disable it by default."] # [macro_export] macro_rules ! trace { ($ ($ tt : tt) *) => { if cfg ! (any (feature = "trace-log" , debug_assertions)) { :: log :: trace ! ($ ($ tt) *) ; } } ; }
};
}
