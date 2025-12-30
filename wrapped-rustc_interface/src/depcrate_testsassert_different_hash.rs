// Generated macro for assert_different_hash (function)
macro_rules! Depcrate_testsassert_different_hash {
() => {
// Module: crate::tests
// Provides: {"assert_different_hash"}
// Dependencies: {}
# [track_caller] fn assert_different_hash (x : & Options , y : & Options) { assert_ne ! (x . dep_tracking_hash (true) , y . dep_tracking_hash (true)) ; assert_ne ! (x . dep_tracking_hash (false) , y . dep_tracking_hash (false)) ; assert_same_clone (x) ; assert_same_clone (y) ; }
};
}
