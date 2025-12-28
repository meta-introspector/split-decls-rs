macro_rules! deps {
    () => {
        ParallelIterator!();
        UnindexedConsumer!();
        IntoIter!();
        WalkTree!();
    };
}

macro_rules! impl_1001 {
    () => {
        deps!();
        impl < S , B , I > ParallelIterator for WalkTree < S , B > where S : Send , B : Fn (& S) -> I + Send + Sync , I : IntoIterator < Item = S , IntoIter : DoubleEndedIterator > + Send , { type Item = S ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { self . 0 . drive_unindexed (consumer) } }
    };
}

impl_1001!()