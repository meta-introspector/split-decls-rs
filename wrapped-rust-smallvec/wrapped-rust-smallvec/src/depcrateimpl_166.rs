// Generated macro for impl_166 (impl)
macro_rules! Depcrateimpl_166 {
() => {
// Module: crate
// Provides: {"impl_166"}
// Dependencies: {}
impl < T , const N : usize > core :: iter :: FromIterator < T > for SmallVec < T , N > { # [inline] fn from_iter < I : IntoIterator < Item = T > > (iter : I) -> Self { # [cfg (feature = "specialization")] { spec_traits :: SpecFromIterator :: < T , _ > :: spec_from_iter (iter . into_iter ()) } # [cfg (not (feature = "specialization"))] { Self :: from_iter_fallback (iter . into_iter ()) } } }
};
}
