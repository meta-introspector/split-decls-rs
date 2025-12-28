macro_rules! deps {
    () => {
        IntoParIter!();
    };
}

macro_rules! impl_143 {
    () => {
        deps!();
        impl < K : Send , V : Send , A : Allocator + Send > ParallelIterator for IntoParIter < K , V , A > { type Item = (K , V) ; # [cfg_attr (feature = "inline-more" , inline)] fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { self . inner . drive_unindexed (consumer) } }
    };
}

impl_143!();