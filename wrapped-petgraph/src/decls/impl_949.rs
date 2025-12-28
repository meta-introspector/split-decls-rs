macro_rules! deps {
    () => {
        ParAllEdgesMut!();
        NodeTrait!();
    };
}

macro_rules! impl_949 {
    () => {
        deps!();
        # [cfg (feature = "rayon")] impl < 'a , N , E , Ty > ParallelIterator for ParAllEdgesMut < 'a , N , E , Ty > where N : NodeTrait + Send + Sync , E : Send , { type Item = (N , N , & 'a mut E) ; fn drive_unindexed < C > (self , c : C) -> C :: Result where C : rayon :: iter :: plumbing :: UnindexedConsumer < Self :: Item > , { self . inner . map (| (& (a , b) , v) | (a , b , v)) . drive_unindexed (c) } fn opt_len (& self) -> Option < usize > { self . inner . opt_len () } }
    };
}

impl_949!()