macro_rules! deps {
    () => {
        IndexedParallelIterator!();
        UpdateConsumer!();
        Producer!();
        ProducerCallback!();
        Consumer!();
        UpdateProducer!();
        Update!();
    };
}

macro_rules! impl_972 {
    () => {
        deps!();
        impl < I , F > IndexedParallelIterator for Update < I , F > where I : IndexedParallelIterator , F : Fn (& mut I :: Item) + Send + Sync , { fn drive < C > (self , consumer : C) -> C :: Result where C : Consumer < Self :: Item > , { let consumer1 = UpdateConsumer :: new (consumer , & self . update_op) ; self . base . drive (consumer1) } fn len (& self) -> usize { self . base . len () } fn with_producer < CB > (self , callback : CB) -> CB :: Output where CB : ProducerCallback < Self :: Item > , { return self . base . with_producer (Callback { callback , update_op : self . update_op , }) ; struct Callback < CB , F > { callback : CB , update_op : F , } impl < T , F , CB > ProducerCallback < T > for Callback < CB , F > where CB : ProducerCallback < T > , F : Fn (& mut T) + Send + Sync , { type Output = CB :: Output ; fn callback < P > (self , base : P) -> CB :: Output where P : Producer < Item = T > , { let producer = UpdateProducer { base , update_op : & self . update_op , } ; self . callback . callback (producer) } } } }
    };
}

impl_972!();