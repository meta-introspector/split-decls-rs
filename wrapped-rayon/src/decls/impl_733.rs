macro_rules! deps {
    () => {
        UnindexedConsumer!();
        ParallelIterator!();
        Once!();
    };
}

macro_rules! impl_733 {
    () => {
        deps!();
        impl < T : Send > ParallelIterator for Once < T > { type Item = T ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { self . drive (consumer) } fn opt_len (& self) -> Option < usize > { Some (1) } }
    };
}

impl_733!();