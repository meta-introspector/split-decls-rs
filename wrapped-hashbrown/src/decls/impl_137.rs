macro_rules! deps {
    () => {
        ParIterMut!();
    };
}

macro_rules! impl_137 {
    () => {
        deps!();
        impl < 'a , K : Sync , V : Send > ParallelIterator for ParIterMut < 'a , K , V > { type Item = (& 'a K , & 'a mut V) ; # [cfg_attr (feature = "inline-more" , inline)] fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { self . inner . map (| x | unsafe { let r = x . as_mut () ; (& r . 0 , & mut r . 1) }) . drive_unindexed (consumer) } }
    };
}

impl_137!()