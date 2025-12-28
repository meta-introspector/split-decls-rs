macro_rules! deps {
    () => {
        UnindexedConsumer!();
        FoldConsumer!();
        Fold!();
        ParallelIterator!();
    };
}

macro_rules! impl_553 {
    () => {
        deps!();
        impl < U , I , ID , F > ParallelIterator for Fold < I , ID , F > where I : ParallelIterator , F : Fn (U , I :: Item) -> U + Sync + Send , ID : Fn () -> U + Sync + Send , U : Send , { type Item = U ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let consumer1 = FoldConsumer { base : consumer , fold_op : & self . fold_op , identity : & self . identity , } ; self . base . drive_unindexed (consumer1) } }
    };
}

impl_553!()