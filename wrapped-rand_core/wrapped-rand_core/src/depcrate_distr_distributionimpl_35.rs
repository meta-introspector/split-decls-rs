// Generated macro for impl_35 (impl)
macro_rules! Depcrate_distr_distributionimpl_35 {
() => {
// Module: crate::distr::distribution
// Provides: {"impl_35"}
// Dependencies: {}
impl < D , F , T , S > Distribution < S > for Map < D , F , T , S > where D : Distribution < T > , F : Fn (T) -> S , { fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> S { (self . func) (self . distr . sample (rng)) } }
};
}
