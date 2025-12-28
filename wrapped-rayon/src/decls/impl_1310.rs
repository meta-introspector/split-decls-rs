macro_rules! deps {
    () => {
        UnindexedConsumer!();
        ParallelIterator!();
        Split!();
        SplitProducer!();
    };
}

macro_rules! impl_1310 {
    () => {
        deps!();
        impl < 'ch , P : Pattern > ParallelIterator for Split < 'ch , P > { type Item = & 'ch str ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let producer = SplitProducer :: new (self . chars , & self . separator) ; bridge_unindexed (producer , consumer) } }
    };
}

impl_1310!();