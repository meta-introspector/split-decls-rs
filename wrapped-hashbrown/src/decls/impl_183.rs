macro_rules! deps {
    () => {
        ParDrain!();
    };
}

macro_rules! impl_183 {
    () => {
        deps!();
        impl < T : Send , A : Allocator + Send + Sync > ParallelIterator for ParDrain < '_ , T , A > { type Item = T ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { self . inner . map (| (k , _) | k) . drive_unindexed (consumer) } }
    };
}

impl_183!()