macro_rules! deps {
    () => {
        Copied!();
        ProducerCallback!();
        Consumer!();
        CopiedProducer!();
        Producer!();
        CopiedConsumer!();
        IndexedParallelIterator!();
    };
}

macro_rules! impl_382 {
    () => {
        deps!();
        impl < 'a , T , I > IndexedParallelIterator for Copied < I > where I : IndexedParallelIterator < Item = & 'a T > , T : 'a + Copy + Send + Sync , { fn drive < C > (self , consumer : C) -> C :: Result where C : Consumer < Self :: Item > , { let consumer1 = CopiedConsumer :: new (consumer) ; self . base . drive (consumer1) } fn len (& self) -> usize { self . base . len () } fn with_producer < CB > (self , callback : CB) -> CB :: Output where CB : ProducerCallback < Self :: Item > , { return self . base . with_producer (Callback { callback }) ; struct Callback < CB > { callback : CB , } impl < 'a , T , CB > ProducerCallback < & 'a T > for Callback < CB > where CB : ProducerCallback < T > , T : 'a + Copy + Send , { type Output = CB :: Output ; fn callback < P > (self , base : P) -> CB :: Output where P : Producer < Item = & 'a T > , { let producer = CopiedProducer { base } ; self . callback . callback (producer) } } } }
    };
}

impl_382!()