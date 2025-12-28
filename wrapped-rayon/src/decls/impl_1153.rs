macro_rules! deps {
    () => {
        ChunksExact!();
        UnindexedConsumer!();
        ParallelIterator!();
    };
}

macro_rules! impl_1153 {
    () => {
        deps!();
        impl < 'data , T : Sync > ParallelIterator for ChunksExact < 'data , T > { type Item = & 'data [T] ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { bridge (self , consumer) } fn opt_len (& self) -> Option < usize > { Some (self . len ()) } }
    };
}

impl_1153!()