macro_rules! deps {
    () => {
        Bucket!();
        ParIterProducer!();
        RawParIter!();
    };
}

macro_rules! impl_164 {
    () => {
        deps!();
        impl < T > ParallelIterator for RawParIter < T > { type Item = Bucket < T > ; # [cfg_attr (feature = "inline-more" , inline)] fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let producer = ParIterProducer { iter : self . iter } ; plumbing :: bridge_unindexed (producer , consumer) } }
    };
}

impl_164!()