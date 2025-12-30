// Generated macro for impl_833 (impl)
macro_rules! Depcrate_ptrimpl_833 {
() => {
// Module: crate::ptr
// Provides: {"impl_833"}
// Dependencies: {}
impl < P : Pointer > DetachablePointer < P > { # [inline] pub fn as_mut (& mut self) -> MutPointer < P :: T > { MutPointer { ptr : self . pointer . as_mut () . unwrap () . as_mut_ptr () , } } }
};
}
