// Generated macro for impl_distribution_via_f32 (macro)
macro_rules! Depcrate_rand_distrimpl_distribution_via_f32 {
() => {
// Module: crate::rand_distr
// Provides: {"impl_distribution_via_f32"}
// Dependencies: {}
macro_rules ! impl_distribution_via_f32 { ($ Ty : ty , $ Distr : ty) => { impl Distribution <$ Ty > for $ Distr { fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> $ Ty { <$ Ty >:: from_f32 (< Self as Distribution < f32 >>:: sample (self , rng)) } } } ; }
};
}
