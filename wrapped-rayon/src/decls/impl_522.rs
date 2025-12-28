macro_rules! deps {
    () => {
        FlatMapIter!();
        UnindexedConsumer!();
        ParallelIterator!();
        FlatMapIterConsumer!();
    };
}

macro_rules! impl_522 {
    () => {
        deps!();
        impl < I , F , SI > ParallelIterator for FlatMapIter < I , F > where I : ParallelIterator , F : Fn (I :: Item) -> SI + Sync + Send , SI : IntoIterator < Item : Send > , { type Item = SI :: Item ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let consumer = FlatMapIterConsumer :: new (consumer , & self . map_op) ; self . base . drive_unindexed (consumer) } }
    };
}

impl_522!()