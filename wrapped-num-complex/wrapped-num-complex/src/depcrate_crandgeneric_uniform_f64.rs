// Generated macro for generic_uniform_f64 (function)
macro_rules! Depcrate_crandgeneric_uniform_f64 {
() => {
// Module: crate::crand
// Provides: {"generic_uniform_f64"}
// Dependencies: {}
# [test] fn generic_uniform_f64 () { use rand :: distributions :: Uniform ; let mut rng = test_rng () ; let re = Uniform :: new (- 100.0 , 0.0) ; let im = Uniform :: new (0.0 , 100.0) ; let dist = ComplexDistribution :: new (re , im) ; for _ in 0 .. 100 { let c = rng . sample (dist) ; assert ! (c . re >= - 100.0 && c . re < 0.0) ; assert ! (c . im >= 0.0 && c . im < 100.0) ; } }
};
}
