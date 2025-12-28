macro_rules! deps {
    () => {
        ParallelIterator!();
        Drain!();
        UnindexedConsumer!();
    };
}

macro_rules! impl_105 {
    () => {
        deps!();
        impl < T : Send > ParallelIterator for Drain < '_ , T > { type Item = T ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { bridge (self , consumer) } fn opt_len (& self) -> Option < usize > { Some (self . len ()) } }
    };
}

impl_105!();