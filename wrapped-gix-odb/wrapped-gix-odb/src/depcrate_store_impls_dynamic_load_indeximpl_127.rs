// Generated macro for impl_127 (impl)
macro_rules! Depcrate_store_impls_dynamic_load_indeximpl_127 {
() => {
// Module: crate::store_impls::dynamic::load_index
// Provides: {"impl_127"}
// Dependencies: {}
impl < 'a > IncOnNewAndDecOnDrop < 'a > { pub fn new (v : & 'a AtomicU16) -> Self { v . fetch_add (1 , Ordering :: SeqCst) ; Self (v) } }
};
}
