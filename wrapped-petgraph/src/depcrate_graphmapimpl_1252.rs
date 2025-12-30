// Generated macro for impl_1252 (impl)
macro_rules! Depcrate_graphmapimpl_1252 {
() => {
// Module: crate::graphmap
// Provides: {"impl_1252"}
// Dependencies: {}
# [cfg (feature = "rayon")] impl < N , E , Ty > IndexedParallelIterator for ParAllEdges < '_ , N , E , Ty > where N : NodeTrait + Send + Sync , E : Sync , { fn drive < C > (self , consumer : C) -> C :: Result where C : rayon :: iter :: plumbing :: Consumer < Self :: Item > , { self . inner . map (| (& (a , b) , v) | (a , b , v)) . drive (consumer) } fn len (& self) -> usize { self . inner . len () } fn with_producer < CB > (self , callback : CB) -> CB :: Output where CB : rayon :: iter :: plumbing :: ProducerCallback < Self :: Item > , { self . inner . map (| (& (a , b) , v) | (a , b , v)) . with_producer (callback) } }
};
}
