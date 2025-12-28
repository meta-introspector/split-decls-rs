macro_rules! deps {
    () => {
        UnindexedConsumer!();
        ParallelIterator!();
        MatchesProducer!();
        Matches!();
    };
}

macro_rules! impl_1332 {
    () => {
        deps!();
        impl < 'ch , P : Pattern > ParallelIterator for Matches < 'ch , P > { type Item = & 'ch str ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let producer = MatchesProducer { chars : self . chars , pattern : & self . pattern , } ; bridge_unindexed (producer , consumer) } }
    };
}

impl_1332!();