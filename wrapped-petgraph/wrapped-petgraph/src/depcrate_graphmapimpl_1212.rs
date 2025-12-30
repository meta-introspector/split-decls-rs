// Generated macro for impl_1212 (impl)
macro_rules! Depcrate_graphmapimpl_1212 {
() => {
// Module: crate::graphmap
// Provides: {"impl_1212"}
// Dependencies: {}
# [doc = " Index `GraphMap` by node pairs to access edge weights."] impl < N , E , Ty , S > IndexMut < (N , N) > for GraphMap < N , E , Ty , S > where N : NodeTrait , Ty : EdgeType , S : BuildHasher , { fn index_mut (& mut self , index : (N , N)) -> & mut E { let index = Self :: edge_key (index . 0 , index . 1) ; self . edge_weight_mut (index . 0 , index . 1) . expect ("GraphMap::index: no such edge") } }
};
}
