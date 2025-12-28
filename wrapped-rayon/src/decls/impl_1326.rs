macro_rules! deps {
    () => {
        UnindexedConsumer!();
        SplitWhitespace!();
        ParallelIterator!();
    };
}

macro_rules! impl_1326 {
    () => {
        deps!();
        impl < 'ch > ParallelIterator for SplitWhitespace < 'ch > { type Item = & 'ch str ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { self . 0 . par_split (char :: is_whitespace) . filter (not_empty) . drive_unindexed (consumer) } }
    };
}

impl_1326!();