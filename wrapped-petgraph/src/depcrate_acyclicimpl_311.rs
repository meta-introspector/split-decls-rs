// Generated macro for impl_311 (impl)
macro_rules! Depcrate_acyclicimpl_311 {
() => {
// Module: crate::acyclic
// Provides: {"impl_311"}
// Dependencies: {}
impl < G : Visitable + EdgeIndexable > EdgeIndexable for Acyclic < G > { fn edge_bound (& self) -> usize { self . inner () . edge_bound () } fn to_index (& self , a : Self :: EdgeId) -> usize { self . inner () . to_index (a) } fn from_index (& self , i : usize) -> Self :: EdgeId { self . inner () . from_index (i) } }
};
}
