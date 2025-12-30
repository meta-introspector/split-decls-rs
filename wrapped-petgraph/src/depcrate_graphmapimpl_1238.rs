// Generated macro for impl_1238 (impl)
macro_rules! Depcrate_graphmapimpl_1238 {
() => {
// Module: crate::graphmap
// Provides: {"impl_1238"}
// Dependencies: {}
impl < 'a , N : 'a , E , Ty , S > visit :: IntoNeighbors for & 'a GraphMap < N , E , Ty , S > where N : Copy + Ord + Hash , Ty : EdgeType , S : BuildHasher , { type Neighbors = Neighbors < 'a , N , Ty > ; fn neighbors (self , n : Self :: NodeId) -> Self :: Neighbors { self . neighbors (n) } }
};
}
