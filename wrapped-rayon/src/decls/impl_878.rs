macro_rules! deps {
    () => {
        ParallelIterator!();
        TakeAnyConsumer!();
        UnindexedConsumer!();
        TakeAny!();
    };
}

macro_rules! impl_878 {
    () => {
        deps!();
        impl < I > ParallelIterator for TakeAny < I > where I : ParallelIterator , { type Item = I :: Item ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let consumer1 = TakeAnyConsumer { base : consumer , count : & AtomicUsize :: new (self . count) , } ; self . base . drive_unindexed (consumer1) } }
    };
}

impl_878!()