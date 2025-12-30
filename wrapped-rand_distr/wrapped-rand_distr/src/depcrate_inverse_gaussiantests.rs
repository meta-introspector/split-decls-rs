// Generated macro for tests (module)
macro_rules! Depcrate_inverse_gaussiantests {
() => {
// Module: crate::inverse_gaussian
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_inverse_gaussian () { let inv_gauss = InverseGaussian :: new (1.0 , 1.0) . unwrap () ; let mut rng = crate :: test :: rng (210) ; for _ in 0 .. 1000 { inv_gauss . sample (& mut rng) ; } } # [test] fn test_inverse_gaussian_invalid_param () { assert ! (InverseGaussian :: new (- 1.0 , 1.0) . is_err ()) ; assert ! (InverseGaussian :: new (- 1.0 , - 1.0) . is_err ()) ; assert ! (InverseGaussian :: new (1.0 , - 1.0) . is_err ()) ; assert ! (InverseGaussian :: new (1.0 , 1.0) . is_ok ()) ; } # [test] fn inverse_gaussian_distributions_can_be_compared () { assert_eq ! (InverseGaussian :: new (1.0 , 2.0) , InverseGaussian :: new (1.0 , 2.0)) ; } }
};
}
