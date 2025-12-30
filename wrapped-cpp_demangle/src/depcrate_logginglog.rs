// Generated macro for log (macro)
macro_rules! Depcrate_logginglog {
() => {
// Module: crate::logging
// Provides: {"log"}
// Dependencies: {}
# [cfg (not (feature = "logging"))] macro_rules ! log { ($ fmt : expr) => { } ; ($ fmt : expr , $ ($ x : tt) *) => { if false { let _ = format ! ($ fmt , $ ($ x) *) ; } } ; }
};
}
