macro_rules! deps {
    () => {
        NodeTrait!();
        ParNodes!();
    };
}

macro_rules! impl_943 {
    () => {
        deps!();
        # [cfg (feature = "rayon")] impl < N > ParallelIterator for ParNodes < '_ , N > where N : NodeTrait + Send + Sync , { type Item = N ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : rayon :: iter :: plumbing :: UnindexedConsumer < Self :: Item > , { self . iter . copied () . drive_unindexed (consumer) } fn opt_len (& self) -> Option < usize > { self . iter . opt_len () } }
    };
}

impl_943!();