macro_rules! deps {
    () => {
        IndexedParallelIterator!();
        ProducerCallback!();
        Consumer!();
        Drain!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl < T : Ord + Send > IndexedParallelIterator for Drain < '_ , T > { fn drive < C > (self , consumer : C) -> C :: Result where C : Consumer < Self :: Item > , { bridge (self , consumer) } fn len (& self) -> usize { self . heap . len () } fn with_producer < CB > (self , callback : CB) -> CB :: Output where CB : ProducerCallback < Self :: Item > , { super :: DrainGuard :: new (self . heap) . par_drain (..) . with_producer (callback) } }
    };
}

impl_35!()