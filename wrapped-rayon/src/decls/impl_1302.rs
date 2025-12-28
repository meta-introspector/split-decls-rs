macro_rules! deps {
    () => {
        UnindexedConsumer!();
        ParallelIterator!();
        Bytes!();
        BytesProducer!();
    };
}

macro_rules! impl_1302 {
    () => {
        deps!();
        impl < 'ch > ParallelIterator for Bytes < 'ch > { type Item = u8 ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { bridge_unindexed (BytesProducer { chars : self . chars } , consumer) } }
    };
}

impl_1302!();