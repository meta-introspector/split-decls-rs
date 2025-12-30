// Generated macro for tests (module)
macro_rules! Depcrate_matchers_not_matchertests {
() => {
// Module: crate::matchers::not_matcher
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: matcher :: MatcherResult ; use crate :: prelude :: * ; use crate :: Result ; use indoc :: indoc ; # [test] fn matches_when_inner_matcher_does_not_match () -> Result < () > { let matcher = not (eq (1)) ; let result = matcher . matches (0) ; verify_that ! (result , eq (MatcherResult :: Match)) } # [test] fn does_not_match_when_inner_matcher_matches () -> Result < () > { let matcher = not (eq (1)) ; let result = matcher . matches (1) ; verify_that ! (result , eq (MatcherResult :: NoMatch)) } # [test] fn match_explanation_references_actual_value () -> Result < () > { let result = verify_that ! (& [1] , not (container_eq ([1]))) ; verify_that ! (result , err (displays_as (contains_substring (indoc ! ("
                Actual: [1],
                  which contains all the elements
                "))))) } }
};
}
