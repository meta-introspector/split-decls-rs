// Generated macro for build (module)
macro_rules! Depcrate_timebuild {
() => {
// Module: crate::time
// Provides: {"build"}
// Dependencies: {}
# [cfg (not (test))] mod build { use std :: time :: Instant ; pub fn now () -> Instant { Instant :: now () } }
};
}
