// Generated macro for test_store_ordering (function)
macro_rules! Depcrate_tests_helpertest_store_ordering {
() => {
// Module: crate::tests::helper
// Provides: {"test_store_ordering"}
// Dependencies: {}
# [track_caller] pub (crate) fn test_store_ordering < T : std :: fmt :: Debug > (f : impl Fn (Ordering) -> T) { for order in STORE_ORDERINGS { f (order) ; } if ! skip_should_panic_test () { assert_eq ! (assert_panic (|| f (Ordering :: Acquire)) , "there is no such thing as an acquire store") ; assert_eq ! (assert_panic (|| f (Ordering :: AcqRel)) , "there is no such thing as an acquire-release store") ; } }
};
}
