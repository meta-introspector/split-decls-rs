// Generated macro for debug (macro)
macro_rules! Depcratedebug {
() => {
// Module: crate
// Provides: {"debug"}
// Dependencies: {}
# [allow (unused)] macro_rules ! debug { ($ ($ x : tt) *) => (# [cfg (feature = "log")] { log :: debug ! ($ ($ x) *) }) }
};
}
