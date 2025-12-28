macro_rules! deps {
    () => {
        ParallelIterator!();
        IntoIter!();
        UnindexedConsumer!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl < T : Send , const N : usize > ParallelIterator for IntoIter < T , N > { type Item = T ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { bridge (self , consumer) } fn opt_len (& self) -> Option < usize > { Some (N) } }
    };
}

impl_21!();