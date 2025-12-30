// Generated macro for impl_316 (impl)
macro_rules! Depcrate_acyclicimpl_316 {
() => {
// Module: crate::acyclic
// Provides: {"impl_316"}
// Dependencies: {}
impl < G : Visitable + NodeIndexable > NodeIndexable for Acyclic < G > { fn node_bound (& self) -> usize { self . inner () . node_bound () } fn to_index (& self , a : Self :: NodeId) -> usize { self . inner () . to_index (a) } fn from_index (& self , i : usize) -> Self :: NodeId { self . inner () . from_index (i) } }
};
}
