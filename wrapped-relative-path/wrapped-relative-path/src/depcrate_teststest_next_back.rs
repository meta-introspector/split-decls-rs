// Generated macro for test_next_back (function)
macro_rules! Depcrate_teststest_next_back {
() => {
// Module: crate::tests
// Provides: {"test_next_back"}
// Dependencies: {}
# [test] fn test_next_back () { use self :: Component :: * ; let mut it = rp ("baz/bar///foo") . components () ; assert_eq ! (Some (Normal ("foo")) , it . next_back ()) ; assert_eq ! (Some (Normal ("bar")) , it . next_back ()) ; assert_eq ! (Some (Normal ("baz")) , it . next_back ()) ; assert_eq ! (None , it . next_back ()) ; }
};
}
