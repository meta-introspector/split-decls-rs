macro_rules! assert_load_ordering {
    () => {
        # [inline] # [cfg_attr (debug_assertions , track_caller)] pub (crate) fn assert_load_ordering (order : Ordering) { match order { Ordering :: Acquire | Ordering :: Relaxed | Ordering :: SeqCst => { } Ordering :: Release => panic ! ("there is no such thing as a release load") , Ordering :: AcqRel => panic ! ("there is no such thing as an acquire-release load") , _ => unreachable ! () , } }
    };
}

assert_load_ordering!()