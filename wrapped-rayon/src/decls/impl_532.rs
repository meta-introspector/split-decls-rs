macro_rules! deps {
    () => {
        FlattenConsumer!();
        Flatten!();
        ParallelIterator!();
        IntoParallelIterator!();
        UnindexedConsumer!();
    };
}

macro_rules! impl_532 {
    () => {
        deps!();
        impl < I > ParallelIterator for Flatten < I > where I : ParallelIterator < Item : IntoParallelIterator > , { type Item = < I :: Item as IntoParallelIterator > :: Item ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let consumer = FlattenConsumer :: new (consumer) ; self . base . drive_unindexed (consumer) } }
    };
}

impl_532!()