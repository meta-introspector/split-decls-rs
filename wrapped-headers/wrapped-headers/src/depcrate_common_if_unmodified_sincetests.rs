// Generated macro for tests (module)
macro_rules! Depcrate_common_if_unmodified_sincetests {
() => {
// Module: crate::common::if_unmodified_since
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use std :: time :: Duration ; # [test] fn precondition_passes () { let newer = SystemTime :: now () ; let exact = newer - Duration :: from_secs (2) ; let older = newer - Duration :: from_secs (4) ; let if_unmod = IfUnmodifiedSince :: from (exact) ; assert ! (! if_unmod . precondition_passes (newer)) ; assert ! (if_unmod . precondition_passes (exact)) ; assert ! (if_unmod . precondition_passes (older)) ; } }
};
}
