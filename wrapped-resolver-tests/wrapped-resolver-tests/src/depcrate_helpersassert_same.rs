// Generated macro for assert_same (function)
macro_rules! Depcrate_helpersassert_same {
() => {
// Module: crate::helpers
// Provides: {"assert_same"}
// Dependencies: {}
# [track_caller] pub fn assert_same < A : PartialEq + Debug > (a : & [A] , b : & [A]) { assert_eq ! (a . len () , b . len () , "not equal\n{a:?}\n{b:?}") ; assert_contains (b , a) ; }
};
}
