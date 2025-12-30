// Generated macro for impl_1246 (impl)
macro_rules! Depcrate_graphmapimpl_1246 {
() => {
// Module: crate::graphmap
// Provides: {"impl_1246"}
// Dependencies: {}
impl < N , E , Ty , S > data :: DataMap for GraphMap < N , E , Ty , S > where N : Copy + Ord + Hash , Ty : EdgeType , S : BuildHasher , { fn edge_weight (& self , id : Self :: EdgeId) -> Option < & Self :: EdgeWeight > { self . edge_weight (id . 0 , id . 1) } fn node_weight (& self , id : Self :: NodeId) -> Option < & Self :: NodeWeight > { self . nodes . get_key_value (& id) . map (| (k , _) | k) } }
};
}
