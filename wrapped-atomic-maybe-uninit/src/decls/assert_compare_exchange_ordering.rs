macro_rules! assert_compare_exchange_ordering {
    () => {
        # [inline] # [cfg_attr (debug_assertions , track_caller)] pub (crate) fn assert_compare_exchange_ordering (success : Ordering , failure : Ordering) { match success { Ordering :: AcqRel | Ordering :: Acquire | Ordering :: Relaxed | Ordering :: Release | Ordering :: SeqCst => { } _ => unreachable ! () , } match failure { Ordering :: Acquire | Ordering :: Relaxed | Ordering :: SeqCst => { } Ordering :: Release => panic ! ("there is no such thing as a release failure ordering") , Ordering :: AcqRel => panic ! ("there is no such thing as an acquire-release failure ordering") , _ => unreachable ! () , } }
    };
}

assert_compare_exchange_ordering!();