macro_rules! deps {
    () => {
        Filter!();
        FilterConsumer!();
        UnindexedConsumer!();
        ParallelIterator!();
    };
}

macro_rules! impl_461 {
    () => {
        deps!();
        impl < I , P > ParallelIterator for Filter < I , P > where I : ParallelIterator , P : Fn (& I :: Item) -> bool + Sync + Send , { type Item = I :: Item ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let consumer1 = FilterConsumer :: new (consumer , & self . filter_op) ; self . base . drive_unindexed (consumer1) } }
    };
}

impl_461!()