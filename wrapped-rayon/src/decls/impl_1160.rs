macro_rules! deps {
    () => {
        Consumer!();
        ChunksMutProducer!();
        ChunksMut!();
        IndexedParallelIterator!();
        ProducerCallback!();
    };
}

macro_rules! impl_1160 {
    () => {
        deps!();
        impl < T : Send > IndexedParallelIterator for ChunksMut < '_ , T > { fn drive < C > (self , consumer : C) -> C :: Result where C : Consumer < Self :: Item > , { bridge (self , consumer) } fn len (& self) -> usize { self . slice . len () . div_ceil (self . chunk_size) } fn with_producer < CB > (self , callback : CB) -> CB :: Output where CB : ProducerCallback < Self :: Item > , { callback . callback (ChunksMutProducer { chunk_size : self . chunk_size , slice : self . slice , }) } }
    };
}

impl_1160!()