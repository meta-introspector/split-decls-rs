macro_rules! deps {
    () => {
        Consumer!();
        IndexedParallelIterator!();
        Producer!();
        MaxLen!();
        ProducerCallback!();
        MaxLenProducer!();
    };
}

macro_rules! impl_670 {
    () => {
        deps!();
        impl < I > IndexedParallelIterator for MaxLen < I > where I : IndexedParallelIterator , { fn drive < C : Consumer < Self :: Item > > (self , consumer : C) -> C :: Result { bridge (self , consumer) } fn len (& self) -> usize { self . base . len () } fn with_producer < CB > (self , callback : CB) -> CB :: Output where CB : ProducerCallback < Self :: Item > , { return self . base . with_producer (Callback { callback , max : self . max , }) ; struct Callback < CB > { callback : CB , max : usize , } impl < T , CB > ProducerCallback < T > for Callback < CB > where CB : ProducerCallback < T > , { type Output = CB :: Output ; fn callback < P > (self , base : P) -> CB :: Output where P : Producer < Item = T > , { let producer = MaxLenProducer { base , max : self . max , } ; self . callback . callback (producer) } } } }
    };
}

impl_670!()