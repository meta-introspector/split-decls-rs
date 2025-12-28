macro_rules! deps {
    () => {
        UnindexedConsumer!();
        ParallelIterator!();
        Chunks!();
    };
}

macro_rules! impl_1146 {
    () => {
        deps!();
        impl < 'data , T : Sync > ParallelIterator for Chunks < 'data , T > { type Item = & 'data [T] ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { bridge (self , consumer) } fn opt_len (& self) -> Option < usize > { Some (self . len ()) } }
    };
}

impl_1146!();