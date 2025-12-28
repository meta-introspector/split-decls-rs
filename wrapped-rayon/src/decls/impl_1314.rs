macro_rules! deps {
    () => {
        SplitInclusive!();
        UnindexedConsumer!();
        SplitInclusiveProducer!();
        ParallelIterator!();
    };
}

macro_rules! impl_1314 {
    () => {
        deps!();
        impl < 'ch , P : Pattern > ParallelIterator for SplitInclusive < 'ch , P > { type Item = & 'ch str ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let producer = SplitInclusiveProducer :: new_incl (self . chars , & self . separator) ; bridge_unindexed (producer , consumer) } }
    };
}

impl_1314!();