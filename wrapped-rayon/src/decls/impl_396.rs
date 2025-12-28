macro_rules! deps {
    () => {
        Empty!();
        UnindexedConsumer!();
        ParallelIterator!();
    };
}

macro_rules! impl_396 {
    () => {
        deps!();
        impl < T : Send > ParallelIterator for Empty < T > { type Item = T ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { self . drive (consumer) } fn opt_len (& self) -> Option < usize > { Some (0) } }
    };
}

impl_396!();