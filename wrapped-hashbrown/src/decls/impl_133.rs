macro_rules! deps {
    () => {
        ParValues!();
    };
}

macro_rules! impl_133 {
    () => {
        deps!();
        impl < 'a , K : Sync , V : Sync > ParallelIterator for ParValues < 'a , K , V > { type Item = & 'a V ; # [cfg_attr (feature = "inline-more" , inline)] fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { self . inner . map (| x | unsafe { & x . as_ref () . 1 }) . drive_unindexed (consumer) } }
    };
}

impl_133!();