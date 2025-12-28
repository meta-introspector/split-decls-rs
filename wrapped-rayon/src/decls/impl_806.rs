macro_rules! deps {
    () => {
        IndexedParallelIterator!();
        Consumer!();
        RepeatN!();
        ProducerCallback!();
    };
}

macro_rules! impl_806 {
    () => {
        deps!();
        impl < T > IndexedParallelIterator for RepeatN < T > where T : Clone + Send , { fn drive < C > (self , consumer : C) -> C :: Result where C : Consumer < Self :: Item > , { bridge (self , consumer) } fn with_producer < CB > (self , callback : CB) -> CB :: Output where CB : ProducerCallback < Self :: Item > , { callback . callback (self . inner) } fn len (& self) -> usize { self . inner . len () } }
    };
}

impl_806!();