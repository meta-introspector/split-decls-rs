macro_rules! deps {
    () => {
        MatchIndicesProducer!();
        UnindexedConsumer!();
        ParallelIterator!();
        MatchIndices!();
    };
}

macro_rules! impl_1336 {
    () => {
        deps!();
        impl < 'ch , P : Pattern > ParallelIterator for MatchIndices < 'ch , P > { type Item = (usize , & 'ch str) ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let producer = MatchIndicesProducer { index : 0 , chars : self . chars , pattern : & self . pattern , } ; bridge_unindexed (producer , consumer) } }
    };
}

impl_1336!()