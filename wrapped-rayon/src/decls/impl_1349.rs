macro_rules! deps {
    () => {
        ParallelIterator!();
        UnindexedConsumer!();
        IntoIter!();
    };
}

macro_rules! impl_1349 {
    () => {
        deps!();
        impl < T : Send > ParallelIterator for IntoIter < T > { type Item = T ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { bridge (self , consumer) } fn opt_len (& self) -> Option < usize > { Some (self . len ()) } }
    };
}

impl_1349!()