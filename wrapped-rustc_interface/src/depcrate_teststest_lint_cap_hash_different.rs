// Generated macro for test_lint_cap_hash_different (function)
macro_rules! Depcrate_teststest_lint_cap_hash_different {
() => {
// Module: crate::tests
// Provides: {"test_lint_cap_hash_different"}
// Dependencies: {}
# [test] fn test_lint_cap_hash_different () { let mut v1 = Options :: default () ; let mut v2 = Options :: default () ; let v3 = Options :: default () ; v1 . lint_cap = Some (Level :: Forbid) ; v2 . lint_cap = Some (Level :: Allow) ; assert_non_crate_hash_different (& v1 , & v2) ; assert_non_crate_hash_different (& v1 , & v3) ; assert_non_crate_hash_different (& v2 , & v3) ; }
};
}
