// Generated macro for impl_1254 (impl)
macro_rules! Depcrate_graphmapimpl_1254 {
() => {
// Module: crate::graphmap
// Provides: {"impl_1254"}
// Dependencies: {}
# [cfg (feature = "rayon")] impl < 'a , N , E , Ty > ParallelIterator for ParAllEdgesMut < 'a , N , E , Ty > where N : NodeTrait + Send + Sync , E : Send , { type Item = (N , N , & 'a mut E) ; fn drive_unindexed < C > (self , c : C) -> C :: Result where C : rayon :: iter :: plumbing :: UnindexedConsumer < Self :: Item > , { self . inner . map (| (& (a , b) , v) | (a , b , v)) . drive_unindexed (c) } fn opt_len (& self) -> Option < usize > { self . inner . opt_len () } }
};
}
