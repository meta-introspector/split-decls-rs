// Generated macro for impl_368 (impl)
macro_rules! Depcrate_adjimpl_368 {
() => {
// Module: crate::adj
// Provides: {"impl_368"}
// Dependencies: {}
impl < E , Ix : IndexType > visit :: NodeIndexable for List < E , Ix > { fn node_bound (& self) -> usize { self . node_count () } # [inline] fn to_index (& self , a : Self :: NodeId) -> usize { a . index () } # [inline] fn from_index (& self , i : usize) -> Self :: NodeId { Ix :: new (i) } }
};
}
