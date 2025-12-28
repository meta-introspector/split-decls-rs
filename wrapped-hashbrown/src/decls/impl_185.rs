macro_rules! deps {
    () => {
        ParIter!();
    };
}

macro_rules! impl_185 {
    () => {
        deps!();
        impl < 'a , T : Sync > ParallelIterator for ParIter < 'a , T > { type Item = & 'a T ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { self . inner . drive_unindexed (consumer) } }
    };
}

impl_185!();