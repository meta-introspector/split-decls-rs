macro_rules! deps {
    () => {
        CharIndicesProducer!();
        CharIndices!();
        ParallelIterator!();
        UnindexedConsumer!();
    };
}

macro_rules! impl_1298 {
    () => {
        deps!();
        impl < 'ch > ParallelIterator for CharIndices < 'ch > { type Item = (usize , char) ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let producer = CharIndicesProducer { index : 0 , chars : self . chars , } ; bridge_unindexed (producer , consumer) } }
    };
}

impl_1298!()