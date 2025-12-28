macro_rules! deps {
    () => {
        ParallelIterator!();
        SplitAsciiWhitespace!();
        UnindexedConsumer!();
    };
}

macro_rules! impl_1329 {
    () => {
        deps!();
        impl < 'ch > ParallelIterator for SplitAsciiWhitespace < 'ch > { type Item = & 'ch str ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { self . 0 . par_split (is_ascii_whitespace) . filter (not_empty) . drive_unindexed (consumer) } }
    };
}

impl_1329!();