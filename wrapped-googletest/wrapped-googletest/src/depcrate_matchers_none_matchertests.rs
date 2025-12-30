// Generated macro for tests (module)
macro_rules! Depcrate_matchers_none_matchertests {
() => {
// Module: crate::matchers::none_matcher
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: matcher :: MatcherResult ; use crate :: prelude :: * ; use crate :: Result ; # [test] fn none_matches_option_with_none () -> Result < () > { let matcher = none () ; let result = matcher . matches (None :: < i32 >) ; verify_that ! (result , eq (MatcherResult :: Match)) } # [test] fn none_does_not_match_option_with_value () -> Result < () > { let matcher = none () ; let result = matcher . matches (Some (0)) ; verify_that ! (result , eq (MatcherResult :: NoMatch)) } # [test] fn none_matches_option_by_ref () -> Result < () > { verify_that ! (None ::< String >, none ()) } # [test] fn none_does_not_match_option_with_value_by_ref () -> Result < () > { verify_that ! (Some ("123" . to_string ()) , not (none ())) } # [test] fn none_describe_match_option_by_ref () -> Result < () > { verify_that ! (Matcher ::<& Option < String >>:: describe (& none () , MatcherResult :: Match) , displays_as (eq ("is none"))) } # [test] fn none_describe_no_match_option_by_ref () -> Result < () > { verify_that ! (Matcher ::<& Option < String >>:: describe (& none () , MatcherResult :: NoMatch) , displays_as (eq ("is some(_)"))) } # [test] fn none_describe_match_option () -> Result < () > { verify_that ! (Matcher ::< Option < i32 >>:: describe (& none () , MatcherResult :: Match) , displays_as (eq ("is none"))) } # [test] fn none_describe_no_match_option () -> Result < () > { verify_that ! (Matcher ::< Option < i32 >>:: describe (& none () , MatcherResult :: NoMatch) , displays_as (eq ("is some(_)"))) } }
};
}
