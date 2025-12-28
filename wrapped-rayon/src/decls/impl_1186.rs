macro_rules! deps {
    () => {
        UnindexedConsumer!();
        ParallelIterator!();
        RChunksMut!();
    };
}

macro_rules! impl_1186 {
    () => {
        deps!();
        impl < 'data , T : Send > ParallelIterator for RChunksMut < 'data , T > { type Item = & 'data mut [T] ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { bridge (self , consumer) } fn opt_len (& self) -> Option < usize > { Some (self . len ()) } }
    };
}

impl_1186!();