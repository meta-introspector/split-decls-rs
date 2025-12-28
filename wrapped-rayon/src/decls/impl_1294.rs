macro_rules! deps {
    () => {
        ParallelIterator!();
        Chars!();
        UnindexedConsumer!();
        CharsProducer!();
    };
}

macro_rules! impl_1294 {
    () => {
        deps!();
        impl < 'ch > ParallelIterator for Chars < 'ch > { type Item = char ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { bridge_unindexed (CharsProducer { chars : self . chars } , consumer) } }
    };
}

impl_1294!()