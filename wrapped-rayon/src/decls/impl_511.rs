macro_rules! deps {
    () => {
        FlatMapConsumer!();
        IntoParallelIterator!();
        ParallelIterator!();
        UnindexedConsumer!();
        FlatMap!();
    };
}

macro_rules! impl_511 {
    () => {
        deps!();
        impl < I , F , PI > ParallelIterator for FlatMap < I , F > where I : ParallelIterator , F : Fn (I :: Item) -> PI + Sync + Send , PI : IntoParallelIterator , { type Item = PI :: Item ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let consumer = FlatMapConsumer :: new (consumer , & self . map_op) ; self . base . drive_unindexed (consumer) } }
    };
}

impl_511!();