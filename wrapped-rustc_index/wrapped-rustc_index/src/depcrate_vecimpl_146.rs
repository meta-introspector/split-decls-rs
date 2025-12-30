// Generated macro for impl_146 (impl)
macro_rules! Depcrate_vecimpl_146 {
() => {
// Module: crate::vec
// Provides: {"impl_146"}
// Dependencies: {}
impl < I : Idx , T > DerefMut for IndexVec < I , T > { # [inline] fn deref_mut (& mut self) -> & mut Self :: Target { self . as_mut_slice () } }
};
}
