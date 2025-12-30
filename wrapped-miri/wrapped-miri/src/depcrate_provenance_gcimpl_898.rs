// Generated macro for impl_898 (impl)
macro_rules! Depcrate_provenance_gcimpl_898 {
() => {
// Module: crate::provenance_gc
// Provides: {"impl_898"}
// Dependencies: {}
impl LiveAllocs < '_ , '_ > { pub fn is_live (& self , id : AllocId) -> bool { self . collected . contains (& id) || self . ecx . is_alloc_live (id) } }
};
}
