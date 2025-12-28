macro_rules! deps {
    () => {
        UnindexedConsumer!();
        ParallelIterator!();
        FlattenIterConsumer!();
        FlattenIter!();
    };
}

macro_rules! impl_542 {
    () => {
        deps!();
        impl < I > ParallelIterator for FlattenIter < I > where I : ParallelIterator < Item : IntoIterator < Item : Send > > , { type Item = < I :: Item as IntoIterator > :: Item ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let consumer = FlattenIterConsumer :: new (consumer) ; self . base . drive_unindexed (consumer) } }
    };
}

impl_542!();