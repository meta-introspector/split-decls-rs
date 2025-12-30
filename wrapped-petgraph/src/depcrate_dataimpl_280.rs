// Generated macro for impl_280 (impl)
macro_rules! Depcrate_dataimpl_280 {
() => {
// Module: crate::data
// Provides: {"impl_280"}
// Dependencies: {}
impl < N , E , Ty , Ix > FromElements for Graph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { fn from_elements < I > (iterable : I) -> Self where Self : Sized , I : IntoIterator < Item = Element < Self :: NodeWeight , Self :: EdgeWeight > > , { from_elements_indexable (iterable) } }
};
}
