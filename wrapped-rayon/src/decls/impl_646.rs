macro_rules! deps {
    () => {
        IntersperseProducer!();
        Intersperse!();
        Consumer!();
        IntersperseConsumer!();
        ProducerCallback!();
        Producer!();
        IndexedParallelIterator!();
    };
}

macro_rules! impl_646 {
    () => {
        deps!();
        impl < I > IndexedParallelIterator for Intersperse < I > where I : IndexedParallelIterator < Item : Clone > , { fn drive < C > (self , consumer : C) -> C :: Result where C : Consumer < Self :: Item > , { let consumer1 = IntersperseConsumer :: new (consumer , self . item) ; self . base . drive (consumer1) } fn len (& self) -> usize { let len = self . base . len () ; if len > 0 { len . checked_add (len - 1) . expect ("overflow") } else { 0 } } fn with_producer < CB > (self , callback : CB) -> CB :: Output where CB : ProducerCallback < Self :: Item > , { let len = self . len () ; return self . base . with_producer (Callback { callback , item : self . item , len , }) ; struct Callback < CB , T > { callback : CB , item : T , len : usize , } impl < T , CB > ProducerCallback < T > for Callback < CB , T > where CB : ProducerCallback < T > , T : Clone + Send , { type Output = CB :: Output ; fn callback < P > (self , base : P) -> CB :: Output where P : Producer < Item = T > , { let producer = IntersperseProducer :: new (base , self . item , self . len) ; self . callback . callback (producer) } } } }
    };
}

impl_646!();