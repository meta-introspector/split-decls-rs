// Generated macro for trace (macro)
macro_rules! Depcratetrace {
() => {
// Module: crate
// Provides: {"trace"}
// Dependencies: {}
# [allow (unused)] macro_rules ! trace { ($ ($ x : tt) *) => (# [cfg (feature = "log")] { log :: trace ! ($ ($ x) *) }) }
};
}
