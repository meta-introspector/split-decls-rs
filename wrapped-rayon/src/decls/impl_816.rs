macro_rules! deps {
    () => {
        Rev!();
        ProducerCallback!();
        IndexedParallelIterator!();
        Producer!();
        Consumer!();
        RevProducer!();
    };
}

macro_rules! impl_816 {
    () => {
        deps!();
        impl < I > IndexedParallelIterator for Rev < I > where I : IndexedParallelIterator , { fn drive < C : Consumer < Self :: Item > > (self , consumer : C) -> C :: Result { bridge (self , consumer) } fn len (& self) -> usize { self . base . len () } fn with_producer < CB > (self , callback : CB) -> CB :: Output where CB : ProducerCallback < Self :: Item > , { let len = self . base . len () ; return self . base . with_producer (Callback { callback , len }) ; struct Callback < CB > { callback : CB , len : usize , } impl < T , CB > ProducerCallback < T > for Callback < CB > where CB : ProducerCallback < T > , { type Output = CB :: Output ; fn callback < P > (self , base : P) -> CB :: Output where P : Producer < Item = T > , { let producer = RevProducer { base , len : self . len , } ; self . callback . callback (producer) } } } }
    };
}

impl_816!()