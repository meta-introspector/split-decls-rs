macro_rules! deps {
    () => {
        ParValuesMut!();
    };
}

macro_rules! impl_140 {
    () => {
        deps!();
        impl < 'a , K : Sync , V : Send > ParallelIterator for ParValuesMut < 'a , K , V > { type Item = & 'a mut V ; # [cfg_attr (feature = "inline-more" , inline)] fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { self . inner . map (| x | unsafe { & mut x . as_mut () . 1 }) . drive_unindexed (consumer) } }
    };
}

impl_140!()