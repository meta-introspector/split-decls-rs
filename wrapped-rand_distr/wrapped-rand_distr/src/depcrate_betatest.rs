// Generated macro for test (module)
macro_rules! Depcrate_betatest {
() => {
// Module: crate::beta
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; # [test] fn test_beta () { let beta = Beta :: new (1.0 , 2.0) . unwrap () ; let mut rng = crate :: test :: rng (201) ; for _ in 0 .. 1000 { beta . sample (& mut rng) ; } } # [test] # [should_panic] fn test_beta_invalid_dof () { Beta :: new (0. , 0.) . unwrap () ; } # [test] fn test_beta_small_param () { let beta = Beta :: < f64 > :: new (1e-3 , 1e-3) . unwrap () ; let mut rng = crate :: test :: rng (206) ; for i in 0 .. 1000 { assert ! (! beta . sample (& mut rng) . is_nan () , "failed at i={i}") ; } } # [test] fn beta_distributions_can_be_compared () { assert_eq ! (Beta :: new (1.0 , 2.0) , Beta :: new (1.0 , 2.0)) ; } }
};
}
