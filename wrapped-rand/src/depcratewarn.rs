// Generated macro for warn (macro)
macro_rules! Depcratewarn {
() => {
// Module: crate
// Provides: {"warn"}
// Dependencies: {}
# [allow (unused)] macro_rules ! warn { ($ ($ x : tt) *) => (# [cfg (feature = "log")] { log :: warn ! ($ ($ x) *) }) }
};
}
