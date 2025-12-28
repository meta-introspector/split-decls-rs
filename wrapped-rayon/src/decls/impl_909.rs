macro_rules! deps {
    () => {
        TryFoldWith!();
        UnindexedConsumer!();
        TryFoldWithConsumer!();
        ParallelIterator!();
    };
}

macro_rules! impl_909 {
    () => {
        deps!();
        impl < U , I , F > ParallelIterator for TryFoldWith < I , U , F > where I : ParallelIterator , F : Fn (U :: Output , I :: Item) -> U + Sync + Send , U : Try < Output : Clone + Send > + Send , { type Item = U ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let consumer1 = TryFoldWithConsumer { base : consumer , item : self . item , fold_op : & self . fold_op , } ; self . base . drive_unindexed (consumer1) } }
    };
}

impl_909!();