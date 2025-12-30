// Generated macro for error (macro)
macro_rules! Depcrateerror {
() => {
// Module: crate
// Provides: {"error"}
// Dependencies: {}
# [allow (unused)] macro_rules ! error { ($ ($ x : tt) *) => (# [cfg (feature = "log")] { log :: error ! ($ ($ x) *) }) }
};
}
