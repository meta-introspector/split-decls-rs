macro_rules! deps {
    () => {
        ZipEq!();
        ProducerCallback!();
        IndexedParallelIterator!();
        Consumer!();
    };
}

macro_rules! impl_1022 {
    () => {
        deps!();
        impl < A , B > IndexedParallelIterator for ZipEq < A , B > where A : IndexedParallelIterator , B : IndexedParallelIterator , { fn drive < C > (self , consumer : C) -> C :: Result where C : Consumer < Self :: Item > , { bridge (self . zip , consumer) } fn len (& self) -> usize { self . zip . len () } fn with_producer < CB > (self , callback : CB) -> CB :: Output where CB : ProducerCallback < Self :: Item > , { self . zip . with_producer (callback) } }
    };
}

impl_1022!()