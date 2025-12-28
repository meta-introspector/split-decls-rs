macro_rules! deps {
    () => {
        ProducerCallback!();
        Consumer!();
        Iter!();
        IterProducer!();
        IndexedParallelIterator!();
    };
}

macro_rules! impl_1248 {
    () => {
        deps!();
        impl < T : Sync > IndexedParallelIterator for Iter < '_ , T > { fn drive < C > (self , consumer : C) -> C :: Result where C : Consumer < Self :: Item > , { bridge (self , consumer) } fn len (& self) -> usize { self . slice . len () } fn with_producer < CB > (self , callback : CB) -> CB :: Output where CB : ProducerCallback < Self :: Item > , { callback . callback (IterProducer { slice : self . slice }) } }
    };
}

impl_1248!()