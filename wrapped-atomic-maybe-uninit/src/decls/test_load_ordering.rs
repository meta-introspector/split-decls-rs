macro_rules! test_load_ordering {
    () => {
        # [track_caller] pub (crate) fn test_load_ordering < T : std :: fmt :: Debug > (f : impl Fn (Ordering) -> T) { for order in LOAD_ORDERINGS { f (order) ; } if ! skip_should_panic_test () { assert_eq ! (assert_panic (|| f (Ordering :: Release)) , "there is no such thing as a release load") ; assert_eq ! (assert_panic (|| f (Ordering :: AcqRel)) , "there is no such thing as an acquire-release load") ; } }
    };
}

test_load_ordering!();