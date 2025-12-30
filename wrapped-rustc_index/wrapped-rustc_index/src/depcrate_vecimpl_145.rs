// Generated macro for impl_145 (impl)
macro_rules! Depcrate_vecimpl_145 {
() => {
// Module: crate::vec
// Provides: {"impl_145"}
// Dependencies: {}
impl < I : Idx , T > Deref for IndexVec < I , T > { type Target = IndexSlice < I , T > ; # [inline] fn deref (& self) -> & Self :: Target { self . as_slice () } }
};
}
