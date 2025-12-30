// Generated macro for tests (module)
macro_rules! Depcrate_weibulltests {
() => {
// Module: crate::weibull
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] # [should_panic] fn invalid () { Weibull :: new (0. , 0.) . unwrap () ; } # [test] fn sample () { let scale = 1.0 ; let shape = 2.0 ; let d = Weibull :: new (scale , shape) . unwrap () ; let mut rng = crate :: test :: rng (1) ; for _ in 0 .. 1000 { let r = d . sample (& mut rng) ; assert ! (r >= 0.) ; } } # [test] fn value_stability () { fn test_samples < F : Float + fmt :: Debug , D : Distribution < F > > (distr : D , zero : F , expected : & [F] ,) { let mut rng = crate :: test :: rng (213) ; let mut buf = [zero ; 4] ; for x in & mut buf { * x = rng . sample (& distr) ; } assert_eq ! (buf , expected) ; } test_samples (Weibull :: new (1.0 , 1.0) . unwrap () , 0f32 , & [0.041495778 , 0.7531094 , 1.4189332 , 0.38386202] ,) ; test_samples (Weibull :: new (2.0 , 0.5) . unwrap () , 0f64 , & [1.1343478702739669 , 0.29470010050655226 , 0.7556151370284702 , 7.877212340241561 ,] ,) ; } # [test] fn weibull_distributions_can_be_compared () { assert_eq ! (Weibull :: new (1.0 , 2.0) , Weibull :: new (1.0 , 2.0)) ; } }
};
}
