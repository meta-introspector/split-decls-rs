// Generated macro for impl_150 (impl)
macro_rules! Depcrate_vecimpl_150 {
() => {
// Module: crate::vec
// Provides: {"impl_150"}
// Dependencies: {}
impl < I : Idx , T > FromIterator < T > for IndexVec < I , T > { # [inline] fn from_iter < J > (iter : J) -> Self where J : IntoIterator < Item = T > , { IndexVec :: from_raw (Vec :: from_iter (iter)) } }
};
}
