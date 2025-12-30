// Generated macro for tests (module)
macro_rules! Depcrate_gumbeltests {
() => {
// Module: crate::gumbel
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] # [should_panic] fn test_zero_scale () { Gumbel :: new (0.0 , 0.0) . unwrap () ; } # [test] # [should_panic] fn test_infinite_scale () { Gumbel :: new (0.0 , f64 :: INFINITY) . unwrap () ; } # [test] # [should_panic] fn test_nan_scale () { Gumbel :: new (0.0 , f64 :: NAN) . unwrap () ; } # [test] # [should_panic] fn test_infinite_location () { Gumbel :: new (f64 :: INFINITY , 1.0) . unwrap () ; } # [test] # [should_panic] fn test_nan_location () { Gumbel :: new (f64 :: NAN , 1.0) . unwrap () ; } # [test] fn test_sample_against_cdf () { fn neg_log_log (x : f64) -> f64 { - (- x . ln ()) . ln () } let location = 0.0 ; let scale = 1.0 ; let iterations = 100_000 ; let increment = 1.0 / iterations as f64 ; let probabilities = [0.1 , 0.2 , 0.3 , 0.4 , 0.5 , 0.6 , 0.7 , 0.8 , 0.9] ; let mut quantiles = [0.0 ; 9] ; for (i , p) in probabilities . iter () . enumerate () { quantiles [i] = neg_log_log (* p) ; } let mut proportions = [0.0 ; 9] ; let d = Gumbel :: new (location , scale) . unwrap () ; let mut rng = crate :: test :: rng (1) ; for _ in 0 .. iterations { let replicate = d . sample (& mut rng) ; for (i , q) in quantiles . iter () . enumerate () { if replicate < * q { proportions [i] += increment ; } } } assert ! (proportions . iter () . zip (& probabilities) . all (| (p_hat , p) | (p_hat - p) . abs () < 0.003)) } # [test] fn gumbel_distributions_can_be_compared () { assert_eq ! (Gumbel :: new (1.0 , 2.0) , Gumbel :: new (1.0 , 2.0)) ; } }
};
}
