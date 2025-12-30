// Generated macro for test (module)
macro_rules! Depcrate_fisher_ftest {
() => {
// Module: crate::fisher_f
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; # [test] fn test_f () { let f = FisherF :: new (2.0 , 32.0) . unwrap () ; let mut rng = crate :: test :: rng (204) ; for _ in 0 .. 1000 { f . sample (& mut rng) ; } } # [test] fn fisher_f_distributions_can_be_compared () { assert_eq ! (FisherF :: new (1.0 , 2.0) , FisherF :: new (1.0 , 2.0)) ; } }
};
}
