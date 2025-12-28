macro_rules! deps {
    () => {
        ParallelIterator!();
        RChunksExactMut!();
        UnindexedConsumer!();
    };
}

macro_rules! impl_1192 {
    () => {
        deps!();
        impl < 'data , T : Send + 'data > ParallelIterator for RChunksExactMut < 'data , T > { type Item = & 'data mut [T] ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { bridge (self , consumer) } fn opt_len (& self) -> Option < usize > { Some (self . len ()) } }
    };
}

impl_1192!()