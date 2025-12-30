// Generated macro for tests (module)
macro_rules! Depcrate_matchers_is_nan_matchertests {
() => {
// Module: crate::matchers::is_nan_matcher
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: prelude :: * ; use crate :: Result ; # [test] fn matches_f32_nan () -> Result < () > { verify_that ! (f32 :: NAN , is_nan ()) } # [test] fn does_not_match_f32_number () -> Result < () > { verify_that ! (0.0f32 , not (is_nan ())) } # [test] fn matches_f64_nan () -> Result < () > { verify_that ! (f64 :: NAN , is_nan ()) } # [test] fn does_not_match_f64_number () -> Result < () > { verify_that ! (0.0f64 , not (is_nan ())) } }
};
}
