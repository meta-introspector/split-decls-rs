// Generated macro for tests (module)
macro_rules! Depcrate_zetatests {
() => {
// Module: crate::zeta
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; fn test_samples < F : Float + fmt :: Debug , D : Distribution < F > > (distr : D , zero : F , expected : & [F]) { let mut rng = crate :: test :: rng (213) ; let mut buf = [zero ; 4] ; for x in & mut buf { * x = rng . sample (& distr) ; } assert_eq ! (buf , expected) ; } # [test] # [should_panic] fn zeta_invalid () { Zeta :: new (1.) . unwrap () ; } # [test] # [should_panic] fn zeta_nan () { Zeta :: new (f64 :: NAN) . unwrap () ; } # [test] fn zeta_sample () { let a = 2.0 ; let d = Zeta :: new (a) . unwrap () ; let mut rng = crate :: test :: rng (1) ; for _ in 0 .. 1000 { let r = d . sample (& mut rng) ; assert ! (r >= 1.) ; } } # [test] fn zeta_small_a () { let a = 1. + 1e-15 ; let d = Zeta :: new (a) . unwrap () ; let mut rng = crate :: test :: rng (2) ; for _ in 0 .. 1000 { let r = d . sample (& mut rng) ; assert ! (r >= 1.) ; } } # [test] fn zeta_value_stability () { test_samples (Zeta :: new (1.5) . unwrap () , 0f32 , & [1.0 , 2.0 , 1.0 , 1.0]) ; test_samples (Zeta :: new (2.0) . unwrap () , 0f64 , & [2.0 , 1.0 , 1.0 , 1.0]) ; } # [test] fn zeta_distributions_can_be_compared () { assert_eq ! (Zeta :: new (1.0) , Zeta :: new (1.0)) ; } }
};
}
