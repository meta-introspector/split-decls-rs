// Generated macro for impl_30 (impl)
macro_rules! Depcrate_alloc_alloc_bytesimpl_30 {
() => {
// Module: crate::alloc::alloc_bytes
// Provides: {"impl_30"}
// Dependencies: {}
impl Clone for MiriAllocBytes { fn clone (& self) -> Self { let bytes : Cow < '_ , [u8] > = Cow :: Borrowed (self) ; let align = Align :: from_bytes (self . layout . align () . to_u64 ()) . unwrap () ; MiriAllocBytes :: from_bytes (bytes , align , self . params . clone ()) } }
};
}
