macro_rules! deps {
    () => {
        SkipAnyConsumer!();
        ParallelIterator!();
        UnindexedConsumer!();
        SkipAny!();
    };
}

macro_rules! impl_827 {
    () => {
        deps!();
        impl < I > ParallelIterator for SkipAny < I > where I : ParallelIterator , { type Item = I :: Item ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let consumer1 = SkipAnyConsumer { base : consumer , count : & AtomicUsize :: new (self . count) , } ; self . base . drive_unindexed (consumer1) } }
    };
}

impl_827!();