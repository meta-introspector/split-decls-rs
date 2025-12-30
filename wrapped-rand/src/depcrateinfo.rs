// Generated macro for info (macro)
macro_rules! Depcrateinfo {
() => {
// Module: crate
// Provides: {"info"}
// Dependencies: {}
# [allow (unused)] macro_rules ! info { ($ ($ x : tt) *) => (# [cfg (feature = "log")] { log :: info ! ($ ($ x) *) }) }
};
}
