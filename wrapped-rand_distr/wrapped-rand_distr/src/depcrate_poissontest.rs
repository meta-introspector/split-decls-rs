// Generated macro for test (module)
macro_rules! Depcrate_poissontest {
() => {
// Module: crate::poisson
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; # [test] # [should_panic] fn test_poisson_invalid_lambda_zero () { Poisson :: new (0.0) . unwrap () ; } # [test] # [should_panic] fn test_poisson_invalid_lambda_infinity () { Poisson :: new (f64 :: INFINITY) . unwrap () ; } # [test] # [should_panic] fn test_poisson_invalid_lambda_neg () { Poisson :: new (- 10.0) . unwrap () ; } # [test] fn poisson_distributions_can_be_compared () { assert_eq ! (Poisson :: new (1.0) , Poisson :: new (1.0)) ; } }
};
}
