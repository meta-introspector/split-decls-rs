macro_rules! deps {
    () => {
        ParIter!();
    };
}

macro_rules! impl_125 {
    () => {
        deps!();
        impl < 'a , K : Sync , V : Sync > ParallelIterator for ParIter < 'a , K , V > { type Item = (& 'a K , & 'a V) ; # [cfg_attr (feature = "inline-more" , inline)] fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { self . inner . map (| x | unsafe { let r = x . as_ref () ; (& r . 0 , & r . 1) }) . drive_unindexed (consumer) } }
    };
}

impl_125!()