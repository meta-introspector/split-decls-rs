macro_rules! deps {
    () => {
        UnindexedConsumer!();
        SplitProducer!();
        ParallelIterator!();
        Split!();
    };
}

macro_rules! impl_1265 {
    () => {
        deps!();
        impl < 'data , T , P > ParallelIterator for Split < 'data , T , P > where P : Fn (& T) -> bool + Sync + Send , T : Sync , { type Item = & 'data [T] ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let producer = SplitProducer :: new (self . slice , & self . separator) ; bridge_unindexed (producer , consumer) } }
    };
}

impl_1265!()