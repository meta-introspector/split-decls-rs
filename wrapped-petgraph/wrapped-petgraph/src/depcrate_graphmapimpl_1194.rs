// Generated macro for impl_1194 (impl)
macro_rules! Depcrate_graphmapimpl_1194 {
() => {
// Module: crate::graphmap
// Provides: {"impl_1194"}
// Dependencies: {}
# [doc = " Create a new `GraphMap` from an iterable of edges."] impl < N , E , Ty , Item , S > FromIterator < Item > for GraphMap < N , E , Ty , S > where Item : IntoWeightedEdge < E , NodeId = N > , N : NodeTrait , Ty : EdgeType , S : BuildHasher + Default , { fn from_iter < I > (iterable : I) -> Self where I : IntoIterator < Item = Item > , { let iter = iterable . into_iter () ; let (low , _) = iter . size_hint () ; let mut g = Self :: with_capacity (0 , low) ; g . extend (iter) ; g } }
};
}
