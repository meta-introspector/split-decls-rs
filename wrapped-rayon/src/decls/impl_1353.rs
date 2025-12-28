macro_rules! deps {
    () => {
        ParallelIterator!();
        UnindexedConsumer!();
        Drain!();
    };
}

macro_rules! impl_1353 {
    () => {
        deps!();
        impl < 'data , T : Send > ParallelIterator for Drain < 'data , T > { type Item = T ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { bridge (self , consumer) } fn opt_len (& self) -> Option < usize > { Some (self . len ()) } }
    };
}

impl_1353!();