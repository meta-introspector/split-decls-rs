macro_rules! deps {
    () => {
        UnindexedConsumer!();
        SkipAnyWhile!();
        ParallelIterator!();
        SkipAnyWhileConsumer!();
    };
}

macro_rules! impl_838 {
    () => {
        deps!();
        impl < I , P > ParallelIterator for SkipAnyWhile < I , P > where I : ParallelIterator , P : Fn (& I :: Item) -> bool + Sync + Send , { type Item = I :: Item ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let consumer1 = SkipAnyWhileConsumer { base : consumer , predicate : & self . predicate , skipping : & AtomicBool :: new (true) , } ; self . base . drive_unindexed (consumer1) } }
    };
}

impl_838!();