macro_rules! deps {
    () => {
        IndexedParallelIterator!();
        ParallelIterator!();
        PositionsConsumer!();
        Positions!();
        UnindexedConsumer!();
    };
}

macro_rules! impl_767 {
    () => {
        deps!();
        impl < I , P > ParallelIterator for Positions < I , P > where I : IndexedParallelIterator , P : Fn (I :: Item) -> bool + Sync + Send , { type Item = usize ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let consumer1 = PositionsConsumer :: new (consumer , & self . predicate , 0) ; self . base . drive (consumer1) } }
    };
}

impl_767!()