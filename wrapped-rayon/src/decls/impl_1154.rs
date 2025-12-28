macro_rules! deps {
    () => {
        ChunksExactProducer!();
        ChunksExact!();
        IndexedParallelIterator!();
        ProducerCallback!();
        Consumer!();
    };
}

macro_rules! impl_1154 {
    () => {
        deps!();
        impl < T : Sync > IndexedParallelIterator for ChunksExact < '_ , T > { fn drive < C > (self , consumer : C) -> C :: Result where C : Consumer < Self :: Item > , { bridge (self , consumer) } fn len (& self) -> usize { self . slice . len () / self . chunk_size } fn with_producer < CB > (self , callback : CB) -> CB :: Output where CB : ProducerCallback < Self :: Item > , { callback . callback (ChunksExactProducer { chunk_size : self . chunk_size , slice : self . slice , }) } }
    };
}

impl_1154!()