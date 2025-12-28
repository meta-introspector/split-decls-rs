macro_rules! deps {
    () => {
        ParIter!();
    };
}

macro_rules! impl_205 {
    () => {
        deps!();
        impl < 'a , T : Sync > ParallelIterator for ParIter < 'a , T > { type Item = & 'a T ; # [cfg_attr (feature = "inline-more" , inline)] fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { self . inner . map (| x | unsafe { x . as_ref () }) . drive_unindexed (consumer) } }
    };
}

impl_205!();