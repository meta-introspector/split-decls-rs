macro_rules! deps {
    () => {
        UnindexedConsumer!();
        ParallelIterator!();
        FilterMapConsumer!();
        FilterMap!();
    };
}

macro_rules! impl_472 {
    () => {
        deps!();
        impl < I , P , R > ParallelIterator for FilterMap < I , P > where I : ParallelIterator , P : Fn (I :: Item) -> Option < R > + Sync + Send , R : Send , { type Item = R ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let consumer = FilterMapConsumer :: new (consumer , & self . filter_op) ; self . base . drive_unindexed (consumer) } }
    };
}

impl_472!()