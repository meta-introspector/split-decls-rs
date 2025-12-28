macro_rules! deps {
    () => {
        UnindexedConsumer!();
        EncodeUtf16Producer!();
        ParallelIterator!();
        EncodeUtf16!();
    };
}

macro_rules! impl_1306 {
    () => {
        deps!();
        impl < 'ch > ParallelIterator for EncodeUtf16 < 'ch > { type Item = u16 ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { bridge_unindexed (EncodeUtf16Producer { chars : self . chars } , consumer) } }
    };
}

impl_1306!();