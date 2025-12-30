// Generated macro for tests (module)
macro_rules! Depcrate_biotests {
() => {
// Module: crate::bio
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: MemBio ; # [test] fn test_mem_bio_get_buf_empty () { let b = MemBio :: new () . unwrap () ; assert_eq ! (b . get_buf () , & []) ; } }
};
}
