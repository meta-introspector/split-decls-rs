// Generated macro for assert_contains (function)
macro_rules! Depcrate_helpersassert_contains {
() => {
// Module: crate::helpers
// Provides: {"assert_contains"}
// Dependencies: {}
# [doc = " Assert `xs` contains `elems`"] # [track_caller] pub fn assert_contains < A : PartialEq + Debug > (xs : & [A] , elems : & [A]) { for elem in elems { assert ! (xs . contains (elem) , "missing element\nset: {xs:?}\nmissing: {elem:?}") ; } }
};
}
