// Generated macro for impl_54 (impl)
macro_rules! Depcrate_multi_dirichletimpl_54 {
() => {
// Module: crate::multi::dirichlet
// Provides: {"impl_54"}
// Dependencies: {}
impl < F > MultiDistribution < F > for DirichletFromBeta < F > where F : Float , StandardNormal : Distribution < F > , Exp1 : Distribution < F > , Open01 : Distribution < F > , { # [inline] fn sample_len (& self) -> usize { self . samplers . len () + 1 } fn sample_to_slice < R : Rng + ? Sized > (& self , rng : & mut R , output : & mut [F]) { assert_eq ! (output . len () , self . sample_len ()) ; let mut acc = F :: one () ; for (s , beta) in output . iter_mut () . zip (self . samplers . iter ()) { let beta_sample = beta . sample (rng) ; * s = acc * beta_sample ; acc = acc * (F :: one () - beta_sample) ; } output [output . len () - 1] = acc ; } }
};
}
