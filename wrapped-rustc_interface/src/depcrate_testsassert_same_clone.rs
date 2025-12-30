// Generated macro for assert_same_clone (function)
macro_rules! Depcrate_testsassert_same_clone {
() => {
// Module: crate::tests
// Provides: {"assert_same_clone"}
// Dependencies: {}
fn assert_same_clone (x : & Options) { assert_eq ! (x . dep_tracking_hash (true) , x . clone () . dep_tracking_hash (true)) ; assert_eq ! (x . dep_tracking_hash (false) , x . clone () . dep_tracking_hash (false)) ; }
};
}
