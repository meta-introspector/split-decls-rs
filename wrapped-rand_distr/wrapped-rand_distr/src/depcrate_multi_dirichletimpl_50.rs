// Generated macro for impl_50 (impl)
macro_rules! Depcrate_multi_dirichletimpl_50 {
() => {
// Module: crate::multi::dirichlet
// Provides: {"impl_50"}
// Dependencies: {}
impl < F > MultiDistribution < F > for DirichletFromGamma < F > where F : Float , StandardNormal : Distribution < F > , Exp1 : Distribution < F > , Open01 : Distribution < F > , { # [inline] fn sample_len (& self) -> usize { self . samplers . len () } fn sample_to_slice < R : Rng + ? Sized > (& self , rng : & mut R , output : & mut [F]) { assert_eq ! (output . len () , self . sample_len ()) ; let mut sum = F :: zero () ; for (s , g) in output . iter_mut () . zip (self . samplers . iter ()) { * s = g . sample (rng) ; sum = sum + * s ; } let invacc = F :: one () / sum ; for s in output . iter_mut () { * s = * s * invacc ; } } }
};
}
