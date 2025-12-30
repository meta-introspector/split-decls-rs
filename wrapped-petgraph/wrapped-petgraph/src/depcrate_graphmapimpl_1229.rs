// Generated macro for impl_1229 (impl)
macro_rules! Depcrate_graphmapimpl_1229 {
() => {
// Module: crate::graphmap
// Provides: {"impl_1229"}
// Dependencies: {}
impl < N , E , Ty , S > visit :: GraphBase for GraphMap < N , E , Ty , S > where N : Copy + PartialEq , S : BuildHasher , { type NodeId = N ; type EdgeId = (N , N) ; }
};
}
