macro_rules! deps {
    () => {
        IndexedParallelIterator!();
        Consumer!();
        DrainProducer!();
        Drain!();
        ProducerCallback!();
    };
}

macro_rules! impl_1354 {
    () => {
        deps!();
        impl < 'data , T : Send > IndexedParallelIterator for Drain < 'data , T > { fn drive < C > (self , consumer : C) -> C :: Result where C : Consumer < Self :: Item > , { bridge (self , consumer) } fn len (& self) -> usize { self . range . len () } fn with_producer < CB > (self , callback : CB) -> CB :: Output where CB : ProducerCallback < Self :: Item > , { unsafe { self . vec . set_len (self . range . start) ; let producer = DrainProducer :: from_vec (self . vec , self . range . len ()) ; callback . callback (producer) } } }
    };
}

impl_1354!()