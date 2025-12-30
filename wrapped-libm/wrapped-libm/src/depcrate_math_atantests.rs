// Generated macro for tests (module)
macro_rules! Depcrate_math_atantests {
() => {
// Module: crate::math::atan
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use core :: f64 :: consts ; use super :: atan ; # [test] fn sanity_check () { for (input , answer) in [(3.0_f64 . sqrt () / 3.0 , consts :: FRAC_PI_6) , (1.0 , consts :: FRAC_PI_4) , (3.0_f64 . sqrt () , consts :: FRAC_PI_3) , (- 3.0_f64 . sqrt () / 3.0 , - consts :: FRAC_PI_6) , (- 1.0 , - consts :: FRAC_PI_4) , (- 3.0_f64 . sqrt () , - consts :: FRAC_PI_3) ,] . iter () { assert ! ((atan (* input) - answer) / answer < 1e-5 , "\natan({:.4}/16) = {:.4}, actual: {}" , input * 16.0 , answer , atan (* input)) ; } } # [test] fn zero () { assert_eq ! (atan (0.0) , 0.0) ; } # [test] fn infinity () { assert_eq ! (atan (f64 :: INFINITY) , consts :: FRAC_PI_2) ; } # [test] fn minus_infinity () { assert_eq ! (atan (f64 :: NEG_INFINITY) , - consts :: FRAC_PI_2) ; } # [test] fn nan () { assert ! (atan (f64 :: NAN) . is_nan ()) ; } }
};
}
