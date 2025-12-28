macro_rules! deps {
    () => {
        UnindexedConsumer!();
        ParallelIterator!();
        TakeAnyWhile!();
        TakeAnyWhileConsumer!();
    };
}

macro_rules! impl_889 {
    () => {
        deps!();
        impl < I , P > ParallelIterator for TakeAnyWhile < I , P > where I : ParallelIterator , P : Fn (& I :: Item) -> bool + Sync + Send , { type Item = I :: Item ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let consumer1 = TakeAnyWhileConsumer { base : consumer , predicate : & self . predicate , taking : & AtomicBool :: new (true) , } ; self . base . drive_unindexed (consumer1) } }
    };
}

impl_889!()