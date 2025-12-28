macro_rules! deps {
    () => {
        ParDrain!();
    };
}

macro_rules! impl_146 {
    () => {
        deps!();
        impl < K : Send , V : Send , A : Allocator + Sync > ParallelIterator for ParDrain < '_ , K , V , A > { type Item = (K , V) ; # [cfg_attr (feature = "inline-more" , inline)] fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { self . inner . drive_unindexed (consumer) } }
    };
}

impl_146!()