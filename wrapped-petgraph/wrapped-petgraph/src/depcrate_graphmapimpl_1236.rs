// Generated macro for impl_1236 (impl)
macro_rules! Depcrate_graphmapimpl_1236 {
() => {
// Module: crate::graphmap
// Provides: {"impl_1236"}
// Dependencies: {}
impl < N , E , Ty , S > visit :: NodeIndexable for GraphMap < N , E , Ty , S > where N : NodeTrait , Ty : EdgeType , S : BuildHasher , { fn node_bound (& self) -> usize { self . node_count () } fn to_index (& self , ix : Self :: NodeId) -> usize { self . nodes . get_index_of (& ix) . expect ("node not found") } fn from_index (& self , ix : usize) -> Self :: NodeId { assert ! (ix < self . nodes . len () , "The requested index {ix} is out-of-bounds.") ; let (& key , _) = self . nodes . get_index (ix) . unwrap () ; key } }
};
}
