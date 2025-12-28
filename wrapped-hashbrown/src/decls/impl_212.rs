macro_rules! deps {
    () => {
        IntoParIter!();
    };
}

macro_rules! impl_212 {
    () => {
        deps!();
        impl < T : Send , A : Allocator + Send > ParallelIterator for IntoParIter < T , A > { type Item = T ; # [cfg_attr (feature = "inline-more" , inline)] fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { self . inner . drive_unindexed (consumer) } }
    };
}

impl_212!()