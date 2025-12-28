macro_rules! deps {
    () => {
        IndexedParallelIterator!();
        Consumer!();
        ProducerCallback!();
        InterleaveShortest!();
    };
}

macro_rules! impl_641 {
    () => {
        deps!();
        impl < I , J > IndexedParallelIterator for InterleaveShortest < I , J > where I : IndexedParallelIterator , J : IndexedParallelIterator < Item = I :: Item > , { fn drive < C > (self , consumer : C) -> C :: Result where C : Consumer < Self :: Item > , { bridge (self , consumer) } fn len (& self) -> usize { self . interleave . len () } fn with_producer < CB > (self , callback : CB) -> CB :: Output where CB : ProducerCallback < Self :: Item > , { self . interleave . with_producer (callback) } }
    };
}

impl_641!()