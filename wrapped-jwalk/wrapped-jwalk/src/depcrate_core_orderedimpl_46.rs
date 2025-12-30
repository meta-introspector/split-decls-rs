// Generated macro for impl_46 (impl)
macro_rules! Depcrate_core_orderedimpl_46 {
() => {
// Module: crate::core::ordered
// Provides: {"impl_46"}
// Dependencies: {}
impl < T > PartialOrd for Ordered < T > { fn partial_cmp (& self , o : & Self) -> Option < Ordering > { Some (self . index_path . cmp (& o . index_path)) } }
};
}
