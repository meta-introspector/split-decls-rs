macro_rules! deps {
    () => {
        ParDrain!();
    };
}

macro_rules! impl_215 {
    () => {
        deps!();
        impl < T : Send , A : Allocator + Sync > ParallelIterator for ParDrain < '_ , T , A > { type Item = T ; # [cfg_attr (feature = "inline-more" , inline)] fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { self . inner . drive_unindexed (consumer) } }
    };
}

impl_215!();