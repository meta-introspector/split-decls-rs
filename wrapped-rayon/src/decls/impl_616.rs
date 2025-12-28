macro_rules! deps {
    () => {
        InspectProducer!();
        Inspect!();
        Consumer!();
        Producer!();
        InspectConsumer!();
        ProducerCallback!();
        IndexedParallelIterator!();
    };
}

macro_rules! impl_616 {
    () => {
        deps!();
        impl < I , F > IndexedParallelIterator for Inspect < I , F > where I : IndexedParallelIterator , F : Fn (& I :: Item) + Sync + Send , { fn drive < C > (self , consumer : C) -> C :: Result where C : Consumer < Self :: Item > , { let consumer1 = InspectConsumer :: new (consumer , & self . inspect_op) ; self . base . drive (consumer1) } fn len (& self) -> usize { self . base . len () } fn with_producer < CB > (self , callback : CB) -> CB :: Output where CB : ProducerCallback < Self :: Item > , { return self . base . with_producer (Callback { callback , inspect_op : self . inspect_op , }) ; struct Callback < CB , F > { callback : CB , inspect_op : F , } impl < T , F , CB > ProducerCallback < T > for Callback < CB , F > where CB : ProducerCallback < T > , F : Fn (& T) + Sync , { type Output = CB :: Output ; fn callback < P > (self , base : P) -> CB :: Output where P : Producer < Item = T > , { let producer = InspectProducer { base , inspect_op : & self . inspect_op , } ; self . callback . callback (producer) } } } }
    };
}

impl_616!()