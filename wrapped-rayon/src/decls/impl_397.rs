macro_rules! deps {
    () => {
        Empty!();
        IndexedParallelIterator!();
        ProducerCallback!();
        Consumer!();
        EmptyProducer!();
    };
}

macro_rules! impl_397 {
    () => {
        deps!();
        impl < T : Send > IndexedParallelIterator for Empty < T > { fn drive < C > (self , consumer : C) -> C :: Result where C : Consumer < Self :: Item > , { consumer . into_folder () . complete () } fn len (& self) -> usize { 0 } fn with_producer < CB > (self , callback : CB) -> CB :: Output where CB : ProducerCallback < Self :: Item > , { callback . callback (EmptyProducer (PhantomData)) } }
    };
}

impl_397!();