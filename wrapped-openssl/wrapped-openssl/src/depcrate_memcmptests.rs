// Generated macro for tests (module)
macro_rules! Depcrate_memcmptests {
() => {
// Module: crate::memcmp
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: eq ; # [test] fn test_eq () { assert ! (eq (& [] , & [])) ; assert ! (eq (& [1] , & [1])) ; assert ! (! eq (& [1 , 2 , 3] , & [1 , 2 , 4])) ; } # [test] # [should_panic] fn test_diff_lens () { eq (& [] , & [1]) ; } }
};
}
