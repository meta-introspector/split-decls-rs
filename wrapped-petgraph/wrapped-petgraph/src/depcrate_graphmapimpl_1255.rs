// Generated macro for impl_1255 (impl)
macro_rules! Depcrate_graphmapimpl_1255 {
() => {
// Module: crate::graphmap
// Provides: {"impl_1255"}
// Dependencies: {}
# [cfg (feature = "rayon")] impl < N , E , Ty > IndexedParallelIterator for ParAllEdgesMut < '_ , N , E , Ty > where N : NodeTrait + Send + Sync , E : Send , { fn drive < C > (self , consumer : C) -> C :: Result where C : rayon :: iter :: plumbing :: Consumer < Self :: Item > , { self . inner . map (| (& (a , b) , v) | (a , b , v)) . drive (consumer) } fn len (& self) -> usize { self . inner . len () } fn with_producer < CB > (self , callback : CB) -> CB :: Output where CB : rayon :: iter :: plumbing :: ProducerCallback < Self :: Item > , { self . inner . map (| (& (a , b) , v) | (a , b , v)) . with_producer (callback) } }
};
}
