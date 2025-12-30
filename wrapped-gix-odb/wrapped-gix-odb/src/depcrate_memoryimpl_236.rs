// Generated macro for impl_236 (impl)
macro_rules! Depcrate_memoryimpl_236 {
() => {
// Module: crate::memory
// Provides: {"impl_236"}
// Dependencies: {}
impl < T > Clone for Proxy < T > where T : Clone , { fn clone (& self) -> Self { Proxy { inner : self . inner . clone () , object_hash : self . object_hash , memory : self . memory . clone () , } } }
};
}
