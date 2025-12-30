// Generated macro for impl_234 (impl)
macro_rules! Depcrate_memoryimpl_234 {
() => {
// Module: crate::memory
// Provides: {"impl_234"}
// Dependencies: {}
impl From < crate :: Handle > for Proxy < crate :: Handle > { fn from (odb : crate :: Handle) -> Self { let object_hash = odb . store . object_hash ; Proxy :: new (odb , object_hash) } }
};
}
