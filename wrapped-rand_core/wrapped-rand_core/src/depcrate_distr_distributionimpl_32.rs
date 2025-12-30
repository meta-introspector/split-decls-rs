// Generated macro for impl_32 (impl)
macro_rules! Depcrate_distr_distributionimpl_32 {
() => {
// Module: crate::distr::distribution
// Provides: {"impl_32"}
// Dependencies: {}
impl < D , R , T > Iterator for Iter < D , R , T > where D : Distribution < T > , R : Rng , { type Item = T ; # [inline (always)] fn next (& mut self) -> Option < T > { Some (self . distr . sample (& mut self . rng)) } fn size_hint (& self) -> (usize , Option < usize >) { (usize :: MAX , None) } }
};
}
