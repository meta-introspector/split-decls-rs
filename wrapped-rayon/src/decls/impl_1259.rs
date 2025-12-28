macro_rules! deps {
    () => {
        ProducerCallback!();
        IterMut!();
        Consumer!();
        IterMutProducer!();
        IndexedParallelIterator!();
    };
}

macro_rules! impl_1259 {
    () => {
        deps!();
        impl < T : Send > IndexedParallelIterator for IterMut < '_ , T > { fn drive < C > (self , consumer : C) -> C :: Result where C : Consumer < Self :: Item > , { bridge (self , consumer) } fn len (& self) -> usize { self . slice . len () } fn with_producer < CB > (self , callback : CB) -> CB :: Output where CB : ProducerCallback < Self :: Item > , { callback . callback (IterMutProducer { slice : self . slice }) } }
    };
}

impl_1259!();