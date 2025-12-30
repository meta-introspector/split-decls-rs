// Generated macro for assert_same_hash (function)
macro_rules! Depcrate_testsassert_same_hash {
() => {
// Module: crate::tests
// Provides: {"assert_same_hash"}
// Dependencies: {}
fn assert_same_hash (x : & Options , y : & Options) { assert_eq ! (x . dep_tracking_hash (true) , y . dep_tracking_hash (true)) ; assert_eq ! (x . dep_tracking_hash (false) , y . dep_tracking_hash (false)) ; assert_same_clone (x) ; assert_same_clone (y) ; }
};
}
