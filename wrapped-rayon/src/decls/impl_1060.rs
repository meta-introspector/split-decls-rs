macro_rules! deps {
    () => {
        ProducerCallback!();
        Consumer!();
        IndexedParallelIterator!();
        Iter!();
    };
}

macro_rules! impl_1060 {
    () => {
        deps!();
        impl < T : IndexedRangeInteger > IndexedParallelIterator for Iter < T > { fn drive < C > (self , consumer : C) -> C :: Result where C : Consumer < T > , { T :: drive (self , consumer) } # [inline] fn len (& self) -> usize { T :: len (self) } fn with_producer < CB > (self , callback : CB) -> CB :: Output where CB : ProducerCallback < T > , { T :: with_producer (self , callback) } }
    };
}

impl_1060!();