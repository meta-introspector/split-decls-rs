macro_rules! deps {
    () => {
        ChunksExactMutProducer!();
        ProducerCallback!();
        IndexedParallelIterator!();
        Consumer!();
        ChunksExactMut!();
    };
}

macro_rules! impl_1166 {
    () => {
        deps!();
        impl < T : Send > IndexedParallelIterator for ChunksExactMut < '_ , T > { fn drive < C > (self , consumer : C) -> C :: Result where C : Consumer < Self :: Item > , { bridge (self , consumer) } fn len (& self) -> usize { self . slice . len () / self . chunk_size } fn with_producer < CB > (self , callback : CB) -> CB :: Output where CB : ProducerCallback < Self :: Item > , { callback . callback (ChunksExactMutProducer { chunk_size : self . chunk_size , slice : self . slice , }) } }
    };
}

impl_1166!()