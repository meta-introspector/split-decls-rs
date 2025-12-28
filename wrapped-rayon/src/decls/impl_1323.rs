macro_rules! deps {
    () => {
        UnindexedConsumer!();
        ParallelIterator!();
        Lines!();
    };
}

macro_rules! impl_1323 {
    () => {
        deps!();
        impl < 'ch > ParallelIterator for Lines < 'ch > { type Item = & 'ch str ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { self . 0 . par_split_terminator ('\n') . map (no_carriage_return) . drive_unindexed (consumer) } }
    };
}

impl_1323!();