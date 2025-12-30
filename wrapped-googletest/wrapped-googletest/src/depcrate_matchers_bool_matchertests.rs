// Generated macro for tests (module)
macro_rules! Depcrate_matchers_bool_matchertests {
() => {
// Module: crate::matchers::bool_matcher
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use crate :: prelude :: * ; use crate :: Result ; # [test] fn match_value () -> Result < () > { verify_that ! (true , is_true ()) ? ; verify_that ! (true , not (is_false ())) ? ; verify_that ! (false , is_false ()) ? ; verify_that ! (false , not (is_true ())) } # [test] fn match_ref () -> Result < () > { let t = true ; let f = false ; verify_that ! (& t , is_true ()) ? ; verify_that ! (& t , not (is_false ())) ? ; verify_that ! (& f , is_false ()) ? ; verify_that ! (& f , not (is_true ())) } # [test] fn describe () { assert_eq ! (is_true () . describe (MatcherResult :: Match) . to_string () , "is true") ; assert_eq ! (is_true () . describe (MatcherResult :: NoMatch) . to_string () , "is false") ; assert_eq ! (is_false () . describe (MatcherResult :: Match) . to_string () , "is false") ; assert_eq ! (is_false () . describe (MatcherResult :: NoMatch) . to_string () , "is true") ; } }
};
}
