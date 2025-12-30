// Generated macro for impl_33 (impl)
macro_rules! Depcrate_alloc_alloc_bytesimpl_33 {
() => {
// Module: crate::alloc::alloc_bytes
// Provides: {"impl_33"}
// Dependencies: {}
impl std :: ops :: DerefMut for MiriAllocBytes { fn deref_mut (& mut self) -> & mut Self :: Target { unsafe { slice :: from_raw_parts_mut (self . ptr , self . layout . size ()) } } }
};
}
