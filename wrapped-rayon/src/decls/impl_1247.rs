macro_rules! deps {
    () => {
        ParallelIterator!();
        Iter!();
        UnindexedConsumer!();
    };
}

macro_rules! impl_1247 {
    () => {
        deps!();
        impl < 'data , T : Sync > ParallelIterator for Iter < 'data , T > { type Item = & 'data T ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { bridge (self , consumer) } fn opt_len (& self) -> Option < usize > { Some (self . len ()) } }
    };
}

impl_1247!()