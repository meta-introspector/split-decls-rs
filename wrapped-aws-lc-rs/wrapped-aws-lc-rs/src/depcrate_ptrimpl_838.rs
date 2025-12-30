// Generated macro for impl_838 (impl)
macro_rules! Depcrate_ptrimpl_838 {
() => {
// Module: crate::ptr
// Provides: {"impl_838"}
// Dependencies: {}
impl < P : Pointer > Drop for DetachablePointer < P > { # [inline] fn drop (& mut self) { if let Some (mut pointer) = self . pointer . take () { pointer . free () ; } } }
};
}
