// Generated macro for tests (module)
macro_rules! Depcrate_paretotests {
() => {
// Module: crate::pareto
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use core :: fmt :: { Debug , Display , LowerExp } ; # [test] # [should_panic] fn invalid () { Pareto :: new (0. , 0.) . unwrap () ; } # [test] fn sample () { let scale = 1.0 ; let shape = 2.0 ; let d = Pareto :: new (scale , shape) . unwrap () ; let mut rng = crate :: test :: rng (1) ; for _ in 0 .. 1000 { let r = d . sample (& mut rng) ; assert ! (r >= scale) ; } } # [test] fn value_stability () { fn test_samples < F : Float + Debug + Display + LowerExp , D : Distribution < F > > (distr : D , thresh : F , expected : & [F] ,) { let mut rng = crate :: test :: rng (213) ; for v in expected { let x = rng . sample (& distr) ; assert_almost_eq ! (x , * v , thresh) ; } } test_samples (Pareto :: new (1f32 , 1.0) . unwrap () , 1e-6 , & [1.0423688 , 2.1235929 , 4.132709 , 1.4679428] ,) ; test_samples (Pareto :: new (2.0 , 0.5) . unwrap () , 1e-14 , & [9.019295276219136 , 4.3097126018270595 , 6.837815045397157 , 105.8826669383772 ,] ,) ; } # [test] fn pareto_distributions_can_be_compared () { assert_eq ! (Pareto :: new (1.0 , 2.0) , Pareto :: new (1.0 , 2.0)) ; } }
};
}
