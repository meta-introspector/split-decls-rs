macro_rules! deps {
    () => {
        RepeatProducer!();
        Repeat!();
        UnindexedConsumer!();
        ParallelIterator!();
    };
}

macro_rules! impl_798 {
    () => {
        deps!();
        impl < T > ParallelIterator for Repeat < T > where T : Clone + Send , { type Item = T ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let producer = RepeatProducer { element : self . element , } ; bridge_unindexed (producer , consumer) } }
    };
}

impl_798!()