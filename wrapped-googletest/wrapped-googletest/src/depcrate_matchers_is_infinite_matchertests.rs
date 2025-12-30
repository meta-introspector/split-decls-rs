// Generated macro for tests (module)
macro_rules! Depcrate_matchers_is_infinite_matchertests {
() => {
// Module: crate::matchers::is_infinite_matcher
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: prelude :: * ; use crate :: Result ; # [test] fn matches_f32_pos_infinity () -> Result < () > { verify_that ! (f32 :: INFINITY , is_infinite ()) } # [test] fn matches_f32_neg_infinity () -> Result < () > { verify_that ! (f32 :: NEG_INFINITY , is_infinite ()) } # [test] fn does_not_match_f32_number () -> Result < () > { verify_that ! (0.0f32 , not (is_infinite ())) } # [test] fn does_not_match_f32_nan () -> Result < () > { verify_that ! (f32 :: NAN , not (is_finite ())) } # [test] fn matches_f64_pos_infinity () -> Result < () > { verify_that ! (f64 :: INFINITY , is_infinite ()) } # [test] fn matches_f64_neg_infinity () -> Result < () > { verify_that ! (f64 :: NEG_INFINITY , is_infinite ()) } # [test] fn does_not_match_f64_number () -> Result < () > { verify_that ! (0.0f64 , not (is_infinite ())) } # [test] fn does_not_match_f64_nan () -> Result < () > { verify_that ! (f64 :: NAN , not (is_finite ())) } }
};
}
