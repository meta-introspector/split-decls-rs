// Generated macro for impl_1239 (impl)
macro_rules! Depcrate_graphmapimpl_1239 {
() => {
// Module: crate::graphmap
// Provides: {"impl_1239"}
// Dependencies: {}
impl < 'a , N : 'a , E , Ty , S > visit :: IntoNeighborsDirected for & 'a GraphMap < N , E , Ty , S > where N : Copy + Ord + Hash , Ty : EdgeType , S : BuildHasher , { type NeighborsDirected = NeighborsDirected < 'a , N , Ty > ; fn neighbors_directed (self , n : N , dir : Direction) -> Self :: NeighborsDirected { self . neighbors_directed (n , dir) } }
};
}
