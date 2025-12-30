// Generated macro for impl_282 (impl)
macro_rules! Depcrate_dataimpl_282 {
() => {
// Module: crate::data
// Provides: {"impl_282"}
// Dependencies: {}
# [cfg (feature = "graphmap")] impl < N , E , Ty , S > FromElements for GraphMap < N , E , Ty , S > where Ty : EdgeType , N : NodeTrait , S : BuildHasher + Default , { fn from_elements < I > (iterable : I) -> Self where Self : Sized , I : IntoIterator < Item = Element < Self :: NodeWeight , Self :: EdgeWeight > > , { from_elements_indexable (iterable) } }
};
}
