// Generated macro for rand (function)
macro_rules! Depcrate_tempfilerand {
() => {
// Module: crate::tempfile
// Provides: {"rand"}
// Dependencies: {}
fn rand () -> u64 { RandomState :: new () . build_hasher () . finish () }
};
}
