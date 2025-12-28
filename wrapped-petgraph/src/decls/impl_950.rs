macro_rules! deps {
    () => {
        ParAllEdgesMut!();
        NodeTrait!();
    };
}

macro_rules! impl_950 {
    () => {
        deps!();
        # [cfg (feature = "rayon")] impl < N , E , Ty > IndexedParallelIterator for ParAllEdgesMut < '_ , N , E , Ty > where N : NodeTrait + Send + Sync , E : Send , { fn drive < C > (self , consumer : C) -> C :: Result where C : rayon :: iter :: plumbing :: Consumer < Self :: Item > , { self . inner . map (| (& (a , b) , v) | (a , b , v)) . drive (consumer) } fn len (& self) -> usize { self . inner . len () } fn with_producer < CB > (self , callback : CB) -> CB :: Output where CB : rayon :: iter :: plumbing :: ProducerCallback < Self :: Item > , { self . inner . map (| (& (a , b) , v) | (a , b , v)) . with_producer (callback) } }
    };
}

impl_950!()