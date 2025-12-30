// Generated macro for tests (module)
macro_rules! Depcrate_normal_inverse_gaussiantests {
() => {
// Module: crate::normal_inverse_gaussian
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_normal_inverse_gaussian () { let norm_inv_gauss = NormalInverseGaussian :: new (2.0 , 1.0) . unwrap () ; let mut rng = crate :: test :: rng (210) ; for _ in 0 .. 1000 { norm_inv_gauss . sample (& mut rng) ; } } # [test] fn test_normal_inverse_gaussian_invalid_param () { assert ! (NormalInverseGaussian :: new (- 1.0 , 1.0) . is_err ()) ; assert ! (NormalInverseGaussian :: new (- 1.0 , - 1.0) . is_err ()) ; assert ! (NormalInverseGaussian :: new (1.0 , 2.0) . is_err ()) ; assert ! (NormalInverseGaussian :: new (2.0 , 1.0) . is_ok ()) ; } # [test] fn normal_inverse_gaussian_distributions_can_be_compared () { assert_eq ! (NormalInverseGaussian :: new (1.0 , 2.0) , NormalInverseGaussian :: new (1.0 , 2.0)) ; } }
};
}
