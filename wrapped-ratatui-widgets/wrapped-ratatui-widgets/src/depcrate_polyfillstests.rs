// Generated macro for tests (module)
macro_rules! Depcrate_polyfillstests {
() => {
// Module: crate::polyfills
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use core :: f64 :: consts :: { FRAC_PI_2 , PI , TAU } ; use super :: * ; extern crate std ; const TEST_VALUES : [f64 ; 24] = [0.0 , 0.5 , - 0.5 , 1.0 , - 1.0 , PI , - PI , FRAC_PI_2 , - FRAC_PI_2 , TAU , - TAU , 9.4248 , - 9.4248 , 0.2528 , - 7.7047 , - 1.1596 , - 2.6095 , 6.8435 , 3.5392 , 5.9725 , 0.9172 , 9.3539 , 2.8843 , - 1.8483 ,] ; const MAX_ERROR : f64 = 0.000_000_000_000_01 ; const TRIG_MAX_ERROR : f64 = 0.002 ; fn assert_with_error (computed : f64 , expected : f64 , max_error : f64) { let delta = (computed - expected) . abs () ; assert ! (delta <= max_error , "error exceeded max value of {max_error}: {computed} vs {expected}") ; } # [test] fn f64_mul_add () { for chunk in TEST_VALUES . chunks (3) { let expected = chunk [0] . mul_add (chunk [1] , chunk [2]) ; let computed = mul_add (chunk [0] , chunk [1] , chunk [2]) ; assert_with_error (computed , expected , MAX_ERROR) ; } } # [test] fn f64_round () { for value in TEST_VALUES { let expected = value . round () ; let computed = round (value) ; assert_with_error (computed , expected , MAX_ERROR) ; } } # [test] fn f64_floor () { for value in TEST_VALUES { let expected = value . floor () ; let computed = floor (value) ; assert_with_error (computed , expected , MAX_ERROR) ; } } # [test] fn f64_sin () { for value in TEST_VALUES { let expected = value . sin () ; let computed = sin (value) ; assert_with_error (computed , expected , TRIG_MAX_ERROR) ; } } # [test] fn f64_cos () { for value in TEST_VALUES { let expected = value . cos () ; let computed = cos (value) ; assert_with_error (computed , expected , TRIG_MAX_ERROR) ; } } }
};
}
