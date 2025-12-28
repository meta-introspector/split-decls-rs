macro_rules! test_compare_exchange_ordering {
    () => {
        pub (crate) fn test_compare_exchange_ordering < T : std :: fmt :: Debug > (f : impl Fn (Ordering , Ordering) -> T ,) { for (success , failure) in COMPARE_EXCHANGE_ORDERINGS { f (success , failure) ; } if ! skip_should_panic_test () { for order in SWAP_ORDERINGS { let msg = assert_panic (| | f (order , Ordering :: AcqRel)) ; assert ! (msg == "there is no such thing as an acquire-release failure ordering" || msg == "there is no such thing as an acquire-release load" , "{}" , msg) ; let msg = assert_panic (| | f (order , Ordering :: Release)) ; assert ! (msg == "there is no such thing as a release failure ordering" || msg == "there is no such thing as a release load" , "{}" , msg) ; } } }
    };
}

test_compare_exchange_ordering!();