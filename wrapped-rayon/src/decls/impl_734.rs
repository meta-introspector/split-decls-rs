macro_rules! deps {
    () => {
        Consumer!();
        ProducerCallback!();
        Once!();
        IndexedParallelIterator!();
    };
}

macro_rules! impl_734 {
    () => {
        deps!();
        impl < T : Send > IndexedParallelIterator for Once < T > { fn drive < C > (self , consumer : C) -> C :: Result where C : Consumer < Self :: Item > , { consumer . into_folder () . consume (self . item) . complete () } fn len (& self) -> usize { 1 } fn with_producer < CB > (self , callback : CB) -> CB :: Output where CB : ProducerCallback < Self :: Item > , { Some (self . item) . into_par_iter () . with_producer (callback) } }
    };
}

impl_734!();