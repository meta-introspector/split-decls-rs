// Generated macro for test_compare_exchange_ordering (function)
macro_rules! Depcrate_tests_helpertest_compare_exchange_ordering {
() => {
// Module: crate::tests::helper
// Provides: {"test_compare_exchange_ordering"}
// Dependencies: {}
pub (crate) fn test_compare_exchange_ordering < T : std :: fmt :: Debug > (f : impl Fn (Ordering , Ordering) -> T ,) { for & (success , failure) in & helper :: COMPARE_EXCHANGE_ORDERINGS { f (success , failure) ; } if ! skip_should_panic_test () { for & order in & helper :: SWAP_ORDERINGS { let msg = assert_panic (| | f (order , Ordering :: AcqRel)) ; assert ! (msg == "there is no such thing as an acquire-release failure ordering" || msg == "there is no such thing as an acquire-release load" , "{}" , msg) ; let msg = assert_panic (| | f (order , Ordering :: Release)) ; assert ! (msg == "there is no such thing as a release failure ordering" || msg == "there is no such thing as a release load" , "{}" , msg) ; } } }
};
}
