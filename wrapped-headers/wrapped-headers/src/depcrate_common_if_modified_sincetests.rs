// Generated macro for tests (module)
macro_rules! Depcrate_common_if_modified_sincetests {
() => {
// Module: crate::common::if_modified_since
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use std :: time :: Duration ; # [test] fn is_modified () { let newer = SystemTime :: now () ; let exact = newer - Duration :: from_secs (2) ; let older = newer - Duration :: from_secs (4) ; let if_mod = IfModifiedSince :: from (exact) ; assert ! (if_mod . is_modified (newer)) ; assert ! (! if_mod . is_modified (exact)) ; assert ! (! if_mod . is_modified (older)) ; } }
};
}
