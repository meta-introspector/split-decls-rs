macro_rules! deps {
    () => {
        ParallelIterator!();
        UnindexedConsumer!();
        SplitInclusiveProducer!();
        SplitInclusiveMut!();
    };
}

macro_rules! impl_1276 {
    () => {
        deps!();
        impl < 'data , T , P > ParallelIterator for SplitInclusiveMut < 'data , T , P > where P : Fn (& T) -> bool + Sync + Send , T : Send , { type Item = & 'data mut [T] ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let producer = SplitInclusiveProducer :: new_incl (self . slice , & self . separator) ; bridge_unindexed (producer , consumer) } }
    };
}

impl_1276!();