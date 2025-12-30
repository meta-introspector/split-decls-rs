// Generated macro for impl_32 (impl)
macro_rules! Depcrate_alloc_alloc_bytesimpl_32 {
() => {
// Module: crate::alloc::alloc_bytes
// Provides: {"impl_32"}
// Dependencies: {}
impl std :: ops :: Deref for MiriAllocBytes { type Target = [u8] ; fn deref (& self) -> & Self :: Target { unsafe { slice :: from_raw_parts (self . ptr , self . layout . size ()) } } }
};
}
