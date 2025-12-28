macro_rules! deps {
    () => {
        IndexedParallelIterator!();
        IntoIter!();
        Consumer!();
        ProducerCallback!();
    };
}

macro_rules! impl_1350 {
    () => {
        deps!();
        impl < T : Send > IndexedParallelIterator for IntoIter < T > { fn drive < C > (self , consumer : C) -> C :: Result where C : Consumer < Self :: Item > , { bridge (self , consumer) } fn len (& self) -> usize { self . vec . len () } fn with_producer < CB > (mut self , callback : CB) -> CB :: Output where CB : ProducerCallback < Self :: Item > , { self . vec . par_drain (..) . with_producer (callback) } }
    };
}

impl_1350!()