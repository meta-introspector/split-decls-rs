// Generated macro for impl_830 (impl)
macro_rules! Depcrate_ptrimpl_830 {
() => {
// Module: crate::ptr
// Provides: {"impl_830"}
// Dependencies: {}
impl < P : Pointer > Drop for ManagedPointer < P > { # [inline] fn drop (& mut self) { self . pointer . free () ; } }
};
}
