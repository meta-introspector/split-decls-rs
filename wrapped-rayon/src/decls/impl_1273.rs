macro_rules! deps {
    () => {
        SplitMut!();
        SplitProducer!();
        UnindexedConsumer!();
        ParallelIterator!();
    };
}

macro_rules! impl_1273 {
    () => {
        deps!();
        impl < 'data , T , P > ParallelIterator for SplitMut < 'data , T , P > where P : Fn (& T) -> bool + Sync + Send , T : Send , { type Item = & 'data mut [T] ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let producer = SplitProducer :: new (self . slice , & self . separator) ; bridge_unindexed (producer , consumer) } }
    };
}

impl_1273!()