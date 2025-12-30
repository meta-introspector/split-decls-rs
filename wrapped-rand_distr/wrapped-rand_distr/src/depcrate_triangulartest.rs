// Generated macro for test (module)
macro_rules! Depcrate_triangulartest {
() => {
// Module: crate::triangular
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; use crate :: utils :: ConstRng ; use rand :: Rng ; # [test] fn test_triangular () { let mut half_rng = ConstRng (0x8000_0000_0000_0000) ; assert_eq ! (half_rng . random ::< f64 > () , 0.5) ; for & (min , max , mode , median) in & [(- 1. , 1. , 0. , 0.) , (1. , 2. , 1. , 2. - 0.5f64 . sqrt ()) , (5. , 25. , 25. , 5. + 200f64 . sqrt ()) , (1e-5 , 1e5 , 1e-3 , 1e5 - 4999999949.5f64 . sqrt ()) , (0. , 1. , 0.9 , 0.45f64 . sqrt ()) , (- 4. , - 0.5 , - 2. , - 4.0 + 3.5f64 . sqrt ()) ,] { # [cfg (feature = "std")] std :: println ! ("{min} {max} {mode} {median}") ; let distr = Triangular :: new (min , max , mode) . unwrap () ; assert_eq ! (distr . sample (& mut half_rng) , median) ; } for & (min , max , mode) in & [(- 1. , 1. , 2.) , (- 1. , 1. , - 2.) , (2. , 1. , 1.)] { assert ! (Triangular :: new (min , max , mode) . is_err ()) ; } } # [test] fn triangular_distributions_can_be_compared () { assert_eq ! (Triangular :: new (1.0 , 3.0 , 2.0) , Triangular :: new (1.0 , 3.0 , 2.0)) ; } }
};
}
