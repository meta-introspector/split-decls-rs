macro_rules! deps {
    () => {
        Drain!();
        UnindexedConsumer!();
        ParallelIterator!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl < T : Ord + Send > ParallelIterator for Drain < '_ , T > { type Item = T ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { bridge (self , consumer) } fn opt_len (& self) -> Option < usize > { Some (self . len ()) } }
    };
}

impl_34!()