macro_rules! deps {
    () => {
        UnindexedConsumer!();
        SplitInclusiveProducer!();
        ParallelIterator!();
        SplitInclusive!();
    };
}

macro_rules! impl_1269 {
    () => {
        deps!();
        impl < 'data , T , P > ParallelIterator for SplitInclusive < 'data , T , P > where P : Fn (& T) -> bool + Sync + Send , T : Sync , { type Item = & 'data [T] ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let producer = SplitInclusiveProducer :: new_incl (self . slice , & self . separator) ; bridge_unindexed (producer , consumer) } }
    };
}

impl_1269!()