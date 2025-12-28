macro_rules! deps {
    () => {
        MapConsumer!();
        ParallelIterator!();
        Map!();
        UnindexedConsumer!();
    };
}

macro_rules! impl_677 {
    () => {
        deps!();
        impl < I , F , R > ParallelIterator for Map < I , F > where I : ParallelIterator , F : Fn (I :: Item) -> R + Sync + Send , R : Send , { type Item = F :: Output ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let consumer1 = MapConsumer :: new (consumer , & self . map_op) ; self . base . drive_unindexed (consumer1) } fn opt_len (& self) -> Option < usize > { self . base . opt_len () } }
    };
}

impl_677!()