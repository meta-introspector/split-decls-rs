macro_rules! deps {
    () => {
        ParKeys!();
    };
}

macro_rules! impl_129 {
    () => {
        deps!();
        impl < 'a , K : Sync , V : Sync > ParallelIterator for ParKeys < 'a , K , V > { type Item = & 'a K ; # [cfg_attr (feature = "inline-more" , inline)] fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { self . inner . map (| x | unsafe { & x . as_ref () . 0 }) . drive_unindexed (consumer) } }
    };
}

impl_129!();