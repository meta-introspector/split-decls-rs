macro_rules! deps {
    () => {
        MatchPosition!();
        ParallelIterator!();
        FindConsumer!();
    };
}

macro_rules! find_last {
    () => {
        deps!();
        pub (super) fn find_last < I , P > (pi : I , find_op : P) -> Option < I :: Item > where I : ParallelIterator , P : Fn (& I :: Item) -> bool + Sync , { let best_found = AtomicUsize :: new (0) ; let consumer = FindConsumer :: new (& find_op , MatchPosition :: Rightmost , & best_found) ; pi . drive_unindexed (consumer) }
    };
}

find_last!()