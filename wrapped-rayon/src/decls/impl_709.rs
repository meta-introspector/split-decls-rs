macro_rules! deps {
    () => {
        Consumer!();
        MapInitConsumer!();
        Producer!();
        MapInit!();
        IndexedParallelIterator!();
        MapInitProducer!();
        ProducerCallback!();
    };
}

macro_rules! impl_709 {
    () => {
        deps!();
        impl < I , INIT , T , F , R > IndexedParallelIterator for MapInit < I , INIT , F > where I : IndexedParallelIterator , INIT : Fn () -> T + Sync + Send , F : Fn (& mut T , I :: Item) -> R + Sync + Send , R : Send , { fn drive < C > (self , consumer : C) -> C :: Result where C : Consumer < Self :: Item > , { let consumer1 = MapInitConsumer :: new (consumer , & self . init , & self . map_op) ; self . base . drive (consumer1) } fn len (& self) -> usize { self . base . len () } fn with_producer < CB > (self , callback : CB) -> CB :: Output where CB : ProducerCallback < Self :: Item > , { return self . base . with_producer (Callback { callback , init : self . init , map_op : self . map_op , }) ; struct Callback < CB , INIT , F > { callback : CB , init : INIT , map_op : F , } impl < T , INIT , U , F , R , CB > ProducerCallback < T > for Callback < CB , INIT , F > where CB : ProducerCallback < R > , INIT : Fn () -> U + Sync , F : Fn (& mut U , T) -> R + Sync , R : Send , { type Output = CB :: Output ; fn callback < P > (self , base : P) -> CB :: Output where P : Producer < Item = T > , { let producer = MapInitProducer { base , init : & self . init , map_op : & self . map_op , } ; self . callback . callback (producer) } } } }
    };
}

impl_709!()