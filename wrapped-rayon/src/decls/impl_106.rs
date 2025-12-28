macro_rules! deps {
    () => {
        ProducerCallback!();
        IndexedParallelIterator!();
        Drain!();
        Consumer!();
    };
}

macro_rules! impl_106 {
    () => {
        deps!();
        impl < T : Send > IndexedParallelIterator for Drain < '_ , T > { fn drive < C > (self , consumer : C) -> C :: Result where C : Consumer < Self :: Item > , { bridge (self , consumer) } fn len (& self) -> usize { self . range . len () } fn with_producer < CB > (self , callback : CB) -> CB :: Output where CB : ProducerCallback < Self :: Item > , { super :: DrainGuard :: new (self . deque) . par_drain (self . range . clone ()) . with_producer (callback) } }
    };
}

impl_106!();