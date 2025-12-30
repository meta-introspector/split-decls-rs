// Generated macro for impl_30 (impl)
macro_rules! Depcrate_distr_distributionimpl_30 {
() => {
// Module: crate::distr::distribution
// Provides: {"impl_30"}
// Dependencies: {}
impl < T , D : Distribution < T > + ? Sized > Distribution < T > for & D { fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> T { (* self) . sample (rng) } }
};
}
