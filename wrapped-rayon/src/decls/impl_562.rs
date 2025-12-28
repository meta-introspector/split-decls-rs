macro_rules! deps {
    () => {
        UnindexedConsumer!();
        FoldWith!();
        FoldWithConsumer!();
        ParallelIterator!();
    };
}

macro_rules! impl_562 {
    () => {
        deps!();
        impl < U , I , F > ParallelIterator for FoldWith < I , U , F > where I : ParallelIterator , F : Fn (U , I :: Item) -> U + Sync + Send , U : Send + Clone , { type Item = U ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let consumer1 = FoldWithConsumer { base : consumer , item : self . item , fold_op : & self . fold_op , } ; self . base . drive_unindexed (consumer1) } }
    };
}

impl_562!()