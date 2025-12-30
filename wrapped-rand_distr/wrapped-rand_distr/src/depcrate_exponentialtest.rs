// Generated macro for test (module)
macro_rules! Depcrate_exponentialtest {
() => {
// Module: crate::exponential
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; # [test] fn test_exp () { let exp = Exp :: new (10.0) . unwrap () ; let mut rng = crate :: test :: rng (221) ; for _ in 0 .. 1000 { assert ! (exp . sample (& mut rng) >= 0.0) ; } } # [test] fn test_zero () { let d = Exp :: new (0.0) . unwrap () ; assert_eq ! (d . sample (& mut crate :: test :: rng (21)) , f64 :: infinity ()) ; } # [test] # [should_panic] fn test_exp_invalid_lambda_neg () { Exp :: new (- 10.0) . unwrap () ; } # [test] # [should_panic] fn test_exp_invalid_lambda_nan () { Exp :: new (f64 :: nan ()) . unwrap () ; } # [test] fn exponential_distributions_can_be_compared () { assert_eq ! (Exp :: new (1.0) , Exp :: new (1.0)) ; } }
};
}
