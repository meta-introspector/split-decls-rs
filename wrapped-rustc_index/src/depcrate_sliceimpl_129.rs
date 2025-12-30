// Generated macro for impl_129 (impl)
macro_rules! Depcrate_sliceimpl_129 {
() => {
// Module: crate::slice
// Provides: {"impl_129"}
// Dependencies: {}
impl < I : Idx , T : Clone > ToOwned for IndexSlice < I , T > { type Owned = IndexVec < I , T > ; fn to_owned (& self) -> IndexVec < I , T > { IndexVec :: from_raw (self . raw . to_owned ()) } fn clone_into (& self , target : & mut IndexVec < I , T >) { self . raw . clone_into (& mut target . raw) } }
};
}
