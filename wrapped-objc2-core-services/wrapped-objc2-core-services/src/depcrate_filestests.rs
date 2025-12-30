// Generated macro for tests (module)
macro_rules! Depcrate_filestests {
() => {
// Module: crate::files
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn alignment () { assert_eq ! (core :: mem :: align_of ::< HFSUniStr255 > () , 2) ; } }
};
}
