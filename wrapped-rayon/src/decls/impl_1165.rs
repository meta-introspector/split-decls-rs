macro_rules! deps {
    () => {
        ParallelIterator!();
        UnindexedConsumer!();
        ChunksExactMut!();
    };
}

macro_rules! impl_1165 {
    () => {
        deps!();
        impl < 'data , T : Send > ParallelIterator for ChunksExactMut < 'data , T > { type Item = & 'data mut [T] ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { bridge (self , consumer) } fn opt_len (& self) -> Option < usize > { Some (self . len ()) } }
    };
}

impl_1165!()