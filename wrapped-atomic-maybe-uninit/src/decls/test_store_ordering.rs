macro_rules! test_store_ordering {
    () => {
        # [track_caller] pub (crate) fn test_store_ordering < T : std :: fmt :: Debug > (f : impl Fn (Ordering) -> T) { for order in STORE_ORDERINGS { f (order) ; } if ! skip_should_panic_test () { assert_eq ! (assert_panic (|| f (Ordering :: Acquire)) , "there is no such thing as an acquire store") ; assert_eq ! (assert_panic (|| f (Ordering :: AcqRel)) , "there is no such thing as an acquire-release store") ; } }
    };
}

test_store_ordering!();