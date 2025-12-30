// Generated macro for impl_149 (impl)
macro_rules! Depcrate_vecimpl_149 {
() => {
// Module: crate::vec
// Provides: {"impl_149"}
// Dependencies: {}
impl < I : Idx , T > Extend < T > for IndexVec < I , T > { # [inline] fn extend < J : IntoIterator < Item = T > > (& mut self , iter : J) { self . raw . extend (iter) ; } # [inline] # [cfg (feature = "nightly")] fn extend_one (& mut self , item : T) { self . raw . push (item) ; } # [inline] # [cfg (feature = "nightly")] fn extend_reserve (& mut self , additional : usize) { self . raw . reserve (additional) ; } }
};
}
