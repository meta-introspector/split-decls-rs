macro_rules! deps {
    () => {
        ChunkByMut!();
        UnindexedConsumer!();
        ChunkByProducer!();
        ParallelIterator!();
    };
}

macro_rules! impl_1141 {
    () => {
        deps!();
        impl < 'data , T , P > ParallelIterator for ChunkByMut < 'data , T , P > where T : Send , P : Fn (& T , & T) -> bool + Send + Sync , { type Item = & 'data mut [T] ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { bridge_unindexed (ChunkByProducer { tail : self . slice . len () , slice : self . slice , pred : & self . pred , marker : PhantomData , } , consumer ,) } }
    };
}

impl_1141!()