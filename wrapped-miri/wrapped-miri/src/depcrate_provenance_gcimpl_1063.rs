// Generated macro for impl_1063 (impl)
macro_rules! Depcrate_provenance_gcimpl_1063 {
() => {
// Module: crate::provenance_gc
// Provides: {"impl_1063"}
// Dependencies: {}
impl LiveAllocs < '_ , '_ > { pub fn is_live (& self , id : AllocId) -> bool { self . collected . contains (& id) || self . ecx . is_alloc_live (id) } }
};
}
