macro_rules! deps {
    () => {
        IntoParIter!();
    };
}

macro_rules! impl_181 {
    () => {
        deps!();
        impl < T : Send , A : Allocator + Send > ParallelIterator for IntoParIter < T , A > { type Item = T ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { self . inner . map (| (k , _) | k) . drive_unindexed (consumer) } }
    };
}

impl_181!();