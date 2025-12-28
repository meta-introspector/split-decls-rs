macro_rules! deps {
    () => {
        Take!();
        Consumer!();
        IndexedParallelIterator!();
        ProducerCallback!();
        Producer!();
    };
}

macro_rules! impl_874 {
    () => {
        deps!();
        impl < I > IndexedParallelIterator for Take < I > where I : IndexedParallelIterator , { fn len (& self) -> usize { self . n } fn drive < C : Consumer < Self :: Item > > (self , consumer : C) -> C :: Result { bridge (self , consumer) } fn with_producer < CB > (self , callback : CB) -> CB :: Output where CB : ProducerCallback < Self :: Item > , { return self . base . with_producer (Callback { callback , n : self . n , }) ; struct Callback < CB > { callback : CB , n : usize , } impl < T , CB > ProducerCallback < T > for Callback < CB > where CB : ProducerCallback < T > , { type Output = CB :: Output ; fn callback < P > (self , base : P) -> CB :: Output where P : Producer < Item = T > , { let (producer , _) = base . split_at (self . n) ; self . callback . callback (producer) } } } }
    };
}

impl_874!();