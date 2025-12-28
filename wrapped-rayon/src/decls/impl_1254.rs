macro_rules! deps {
    () => {
        Consumer!();
        IndexedParallelIterator!();
        ProducerCallback!();
        WindowsProducer!();
        Windows!();
    };
}

macro_rules! impl_1254 {
    () => {
        deps!();
        impl < T : Sync > IndexedParallelIterator for Windows < '_ , T > { fn drive < C > (self , consumer : C) -> C :: Result where C : Consumer < Self :: Item > , { bridge (self , consumer) } fn len (& self) -> usize { assert ! (self . window_size >= 1) ; self . slice . len () . saturating_sub (self . window_size - 1) } fn with_producer < CB > (self , callback : CB) -> CB :: Output where CB : ProducerCallback < Self :: Item > , { callback . callback (WindowsProducer { window_size : self . window_size , slice : self . slice , }) } }
    };
}

impl_1254!();