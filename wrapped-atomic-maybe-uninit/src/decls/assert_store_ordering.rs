macro_rules! assert_store_ordering {
    () => {
        # [inline] # [cfg_attr (debug_assertions , track_caller)] pub (crate) fn assert_store_ordering (order : Ordering) { match order { Ordering :: Release | Ordering :: Relaxed | Ordering :: SeqCst => { } Ordering :: Acquire => panic ! ("there is no such thing as an acquire store") , Ordering :: AcqRel => panic ! ("there is no such thing as an acquire-release store") , _ => unreachable ! () , } }
    };
}

assert_store_ordering!();