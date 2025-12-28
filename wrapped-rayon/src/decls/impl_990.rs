macro_rules! deps {
    () => {
        IntoIter!();
        WalkTreePrefixProducer!();
        UnindexedConsumer!();
        ParallelIterator!();
        WalkTreePrefix!();
    };
}

macro_rules! impl_990 {
    () => {
        deps!();
        impl < S , B , I > ParallelIterator for WalkTreePrefix < S , B > where S : Send , B : Fn (& S) -> I + Send + Sync , I : IntoIterator < Item = S , IntoIter : DoubleEndedIterator > , { type Item = S ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let producer = WalkTreePrefixProducer { to_explore : once (self . initial_state) . collect () , seen : Vec :: new () , children_of : & self . children_of , } ; bridge_unindexed (producer , consumer) } }
    };
}

impl_990!();