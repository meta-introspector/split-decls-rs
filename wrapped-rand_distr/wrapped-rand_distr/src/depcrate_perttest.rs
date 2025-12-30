// Generated macro for test (module)
macro_rules! Depcrate_perttest {
() => {
// Module: crate::pert
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; # [test] fn test_pert () { for & (min , max , mode) in & [(- 1. , 1. , 0.) , (1. , 2. , 1.) , (5. , 25. , 25.)] { let _distr = Pert :: new (min , max) . with_mode (mode) . unwrap () ; } for & (min , max , mode) in & [(- 1. , 1. , 2.) , (- 1. , 1. , - 2.) , (2. , 1. , 1.)] { assert ! (Pert :: new (min , max) . with_mode (mode) . is_err ()) ; } } # [test] fn distributions_can_be_compared () { let (min , mode , max , shape) = (1.0 , 2.0 , 3.0 , 4.0) ; let p1 = Pert :: new (min , max) . with_mode (mode) . unwrap () ; let mean = (min + shape * mode + max) / (shape + 2.0) ; let p2 = Pert :: new (min , max) . with_mean (mean) . unwrap () ; assert_eq ! (p1 , p2) ; } # [test] fn mode_almost_half_range () { assert ! (Pert :: new (0.0f32 , 0.48258883) . with_mode (0.24129441) . is_ok ()) ; } # [test] fn almost_symmetric_about_zero () { let distr = Pert :: new (- 10f32 , 10f32) . with_mode (f32 :: EPSILON) ; assert ! (distr . is_ok ()) ; } # [test] fn almost_symmetric () { let distr = Pert :: new (0f32 , 2f32) . with_mode (1f32 + f32 :: EPSILON) ; assert ! (distr . is_ok ()) ; } }
};
}
