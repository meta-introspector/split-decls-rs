macro_rules! deps {
    () => {
        UnindexedConsumer!();
        TryFold!();
        TryFoldConsumer!();
        ParallelIterator!();
    };
}

macro_rules! impl_900 {
    () => {
        deps!();
        impl < U , I , ID , F > ParallelIterator for TryFold < I , U , ID , F > where I : ParallelIterator , F : Fn (U :: Output , I :: Item) -> U + Sync + Send , ID : Fn () -> U :: Output + Sync + Send , U : Try + Send , { type Item = U ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let consumer1 = TryFoldConsumer { base : consumer , identity : & self . identity , fold_op : & self . fold_op , marker : PhantomData , } ; self . base . drive_unindexed (consumer1) } }
    };
}

impl_900!()