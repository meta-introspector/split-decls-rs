// Generated macro for impl_281 (impl)
macro_rules! Depcrate_dataimpl_281 {
() => {
// Module: crate::data
// Provides: {"impl_281"}
// Dependencies: {}
# [cfg (feature = "stable_graph")] impl < N , E , Ty , Ix > FromElements for StableGraph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { fn from_elements < I > (iterable : I) -> Self where Self : Sized , I : IntoIterator < Item = Element < Self :: NodeWeight , Self :: EdgeWeight > > , { from_elements_indexable (iterable) } }
};
}
