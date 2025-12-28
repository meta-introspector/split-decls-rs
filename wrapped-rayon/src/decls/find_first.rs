macro_rules! deps {
    () => {
        MatchPosition!();
        ParallelIterator!();
        FindConsumer!();
    };
}

macro_rules! find_first {
    () => {
        deps!();
        pub (super) fn find_first < I , P > (pi : I , find_op : P) -> Option < I :: Item > where I : ParallelIterator , P : Fn (& I :: Item) -> bool + Sync , { let best_found = AtomicUsize :: new (usize :: MAX) ; let consumer = FindConsumer :: new (& find_op , MatchPosition :: Leftmost , & best_found) ; pi . drive_unindexed (consumer) }
    };
}

find_first!()