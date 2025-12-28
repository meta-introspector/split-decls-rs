macro_rules! deps {
    () => {
        WalkTreePostfix!();
        ParallelIterator!();
        WalkTreePostfixProducer!();
        UnindexedConsumer!();
    };
}

macro_rules! impl_996 {
    () => {
        deps!();
        impl < S , B , I > ParallelIterator for WalkTreePostfix < S , B > where S : Send , B : Fn (& S) -> I + Send + Sync , I : IntoIterator < Item = S > , { type Item = S ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let producer = WalkTreePostfixProducer { to_explore : once (self . initial_state) . collect () , seen : Vec :: new () , children_of : & self . children_of , } ; bridge_unindexed (producer , consumer) } }
    };
}

impl_996!();