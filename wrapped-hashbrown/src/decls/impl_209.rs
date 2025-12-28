macro_rules! deps {
    () => {
        ParIterMut!();
    };
}

macro_rules! impl_209 {
    () => {
        deps!();
        impl < 'a , T : Send > ParallelIterator for ParIterMut < 'a , T > { type Item = & 'a mut T ; # [cfg_attr (feature = "inline-more" , inline)] fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { self . inner . map (| x | unsafe { x . as_mut () }) . drive_unindexed (consumer) } }
    };
}

impl_209!()