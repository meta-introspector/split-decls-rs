macro_rules! deps {
    () => {
        SplitProducer!();
        Split!();
        UnindexedConsumer!();
        ParallelIterator!();
    };
}

macro_rules! impl_849 {
    () => {
        deps!();
        impl < D , S > ParallelIterator for Split < D , S > where D : Send , S : Fn (D) -> (D , Option < D >) + Sync + Send , { type Item = D ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let producer = SplitProducer { data : self . data , splitter : & self . splitter , } ; bridge_unindexed (producer , consumer) } }
    };
}

impl_849!()