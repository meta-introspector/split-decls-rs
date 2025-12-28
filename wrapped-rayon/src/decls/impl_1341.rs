macro_rules! deps {
    () => {
        ParallelIterator!();
        UnindexedConsumer!();
        Drain!();
    };
}

macro_rules! impl_1341 {
    () => {
        deps!();
        impl < 'a > ParallelIterator for Drain < 'a > { type Item = char ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { self . string [self . range . clone ()] . par_chars () . drive_unindexed (consumer) } }
    };
}

impl_1341!()