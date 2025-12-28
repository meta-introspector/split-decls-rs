macro_rules! deps {
    () => {
        ParallelIterator!();
        IntoIter!();
        UnindexedConsumer!();
    };
}

macro_rules! impl_1040 {
    () => {
        deps!();
        impl < T : Send > ParallelIterator for IntoIter < T > { type Item = T ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { self . drive (consumer) } fn opt_len (& self) -> Option < usize > { Some (self . len ()) } }
    };
}

impl_1040!()