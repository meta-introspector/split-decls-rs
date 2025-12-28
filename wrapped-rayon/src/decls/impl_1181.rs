macro_rules! deps {
    () => {
        Consumer!();
        RChunksExact!();
        IndexedParallelIterator!();
        ProducerCallback!();
        RChunksExactProducer!();
    };
}

macro_rules! impl_1181 {
    () => {
        deps!();
        impl < T : Sync > IndexedParallelIterator for RChunksExact < '_ , T > { fn drive < C > (self , consumer : C) -> C :: Result where C : Consumer < Self :: Item > , { bridge (self , consumer) } fn len (& self) -> usize { self . slice . len () / self . chunk_size } fn with_producer < CB > (self , callback : CB) -> CB :: Output where CB : ProducerCallback < Self :: Item > , { callback . callback (RChunksExactProducer { chunk_size : self . chunk_size , slice : self . slice , }) } }
    };
}

impl_1181!()