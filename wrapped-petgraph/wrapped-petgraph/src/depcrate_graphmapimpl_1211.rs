// Generated macro for impl_1211 (impl)
macro_rules! Depcrate_graphmapimpl_1211 {
() => {
// Module: crate::graphmap
// Provides: {"impl_1211"}
// Dependencies: {}
# [doc = " Index `GraphMap` by node pairs to access edge weights."] impl < N , E , Ty , S > Index < (N , N) > for GraphMap < N , E , Ty , S > where N : NodeTrait , Ty : EdgeType , S : BuildHasher , { type Output = E ; fn index (& self , index : (N , N)) -> & E { let index = Self :: edge_key (index . 0 , index . 1) ; self . edge_weight (index . 0 , index . 1) . expect ("GraphMap::index: no such edge") } }
};
}
