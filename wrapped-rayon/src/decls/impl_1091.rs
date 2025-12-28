macro_rules! deps {
    () => {
        Iter!();
        IndexedParallelIterator!();
        ProducerCallback!();
        Consumer!();
    };
}

macro_rules! impl_1091 {
    () => {
        deps!();
        impl < T : IndexedRangeInteger > IndexedParallelIterator for Iter < T > { fn drive < C > (self , consumer : C) -> C :: Result where C : Consumer < T > , { T :: drive (self , consumer) } # [inline] fn len (& self) -> usize { T :: len (self) } fn with_producer < CB > (self , callback : CB) -> CB :: Output where CB : ProducerCallback < T > , { T :: with_producer (self , callback) } }
    };
}

impl_1091!();