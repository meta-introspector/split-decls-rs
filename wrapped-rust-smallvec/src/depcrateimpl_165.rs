// Generated macro for impl_165 (impl)
macro_rules! Depcrateimpl_165 {
() => {
// Module: crate
// Provides: {"impl_165"}
// Dependencies: {}
impl < 'a , T : Clone + 'a , const N : usize > Extend < & 'a T > for SmallVec < T , N > { # [inline] fn extend < I : IntoIterator < Item = & 'a T > > (& mut self , iter : I) { # [cfg (feature = "specialization")] { spec_traits :: SpecExtend :: < & 'a T , _ > :: spec_extend (self , iter . into_iter ()) ; } # [cfg (not (feature = "specialization"))] { self . extend_fallback (iter . into_iter () . cloned ()) ; } } }
};
}
