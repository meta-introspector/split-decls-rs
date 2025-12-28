macro_rules! deps {
    () => {
        ChunkByProducer!();
        UnindexedConsumer!();
        ParallelIterator!();
        ChunkBy!();
    };
}

macro_rules! impl_1137 {
    () => {
        deps!();
        impl < 'data , T , P > ParallelIterator for ChunkBy < 'data , T , P > where T : Sync , P : Fn (& T , & T) -> bool + Send + Sync , { type Item = & 'data [T] ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { bridge_unindexed (ChunkByProducer { tail : self . slice . len () , slice : self . slice , pred : & self . pred , marker : PhantomData , } , consumer ,) } }
    };
}

impl_1137!();