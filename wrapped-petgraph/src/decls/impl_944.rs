macro_rules! deps {
    () => {
        NodeTrait!();
        ParNodes!();
    };
}

macro_rules! impl_944 {
    () => {
        deps!();
        # [cfg (feature = "rayon")] impl < N > IndexedParallelIterator for ParNodes < '_ , N > where N : NodeTrait + Send + Sync , { fn drive < C > (self , consumer : C) -> C :: Result where C : rayon :: iter :: plumbing :: Consumer < Self :: Item > , { self . iter . copied () . drive (consumer) } fn len (& self) -> usize { self . iter . len () } fn with_producer < CB > (self , callback : CB) -> CB :: Output where CB : rayon :: iter :: plumbing :: ProducerCallback < Self :: Item > , { self . iter . copied () . with_producer (callback) } }
    };
}

impl_944!();