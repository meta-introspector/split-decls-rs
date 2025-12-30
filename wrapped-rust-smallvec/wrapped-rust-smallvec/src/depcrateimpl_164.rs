// Generated macro for impl_164 (impl)
macro_rules! Depcrateimpl_164 {
() => {
// Module: crate
// Provides: {"impl_164"}
// Dependencies: {}
impl < T , const N : usize > Extend < T > for SmallVec < T , N > { # [inline] fn extend < I : IntoIterator < Item = T > > (& mut self , iter : I) { # [cfg (feature = "specialization")] { spec_traits :: SpecExtend :: < T , _ > :: spec_extend (self , iter . into_iter ()) ; } # [cfg (not (feature = "specialization"))] { self . extend_fallback (iter) ; } } }
};
}
