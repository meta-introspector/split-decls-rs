macro_rules! deps {
    () => {
        UnindexedConsumer!();
        SplitTerminator!();
        SplitTerminatorProducer!();
        ParallelIterator!();
    };
}

macro_rules! impl_1319 {
    () => {
        deps!();
        impl < 'ch , P : Pattern > ParallelIterator for SplitTerminator < 'ch , P > { type Item = & 'ch str ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let producer = SplitTerminatorProducer :: new (self . chars , & self . terminator) ; bridge_unindexed (producer , consumer) } }
    };
}

impl_1319!()