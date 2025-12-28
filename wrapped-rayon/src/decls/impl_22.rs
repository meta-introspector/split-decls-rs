macro_rules! deps {
    () => {
        IndexedParallelIterator!();
        Consumer!();
        ProducerCallback!();
        DrainProducer!();
        IntoIter!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl < T : Send , const N : usize > IndexedParallelIterator for IntoIter < T , N > { fn drive < C > (self , consumer : C) -> C :: Result where C : Consumer < Self :: Item > , { bridge (self , consumer) } fn len (& self) -> usize { N } fn with_producer < CB > (self , callback : CB) -> CB :: Output where CB : ProducerCallback < Self :: Item > , { unsafe { let mut array = ManuallyDrop :: new (self . array) ; let producer = DrainProducer :: new (array . as_mut_slice ()) ; callback . callback (producer) } } }
    };
}

impl_22!();