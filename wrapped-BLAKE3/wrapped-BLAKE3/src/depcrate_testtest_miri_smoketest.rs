// Generated macro for test_miri_smoketest (function)
macro_rules! Depcrate_testtest_miri_smoketest {
() => {
// Module: crate::test
// Provides: {"test_miri_smoketest"}
// Dependencies: {}
# [test] fn test_miri_smoketest () { let mut hasher = crate :: Hasher :: new_derive_key ("Miri smoketest") ; hasher . update (b"foo") ; # [cfg (feature = "std")] hasher . update_reader (& b"bar" [..]) . unwrap () ; assert_eq ! (hasher . finalize () , hasher . finalize ()) ; let mut reader = hasher . finalize_xof () ; reader . set_position (999999) ; reader . fill (& mut [0]) ; }
};
}
