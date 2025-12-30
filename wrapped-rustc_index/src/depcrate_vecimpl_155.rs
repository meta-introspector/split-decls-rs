// Generated macro for impl_155 (impl)
macro_rules! Depcrate_vecimpl_155 {
() => {
// Module: crate::vec
// Provides: {"impl_155"}
// Dependencies: {}
impl < I : Idx , T , const N : usize > From < [T ; N] > for IndexVec < I , T > { # [inline] fn from (array : [T ; N]) -> Self { IndexVec :: from_raw (array . into ()) } }
};
}
