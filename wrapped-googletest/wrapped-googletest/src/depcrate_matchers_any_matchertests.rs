// Generated macro for tests (module)
macro_rules! Depcrate_matchers_any_matchertests {
() => {
// Module: crate::matchers::any_matcher
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: matcher :: MatcherResult ; use crate :: prelude :: * ; use crate :: Result ; use indoc :: indoc ; # [test] fn description_shows_more_than_one_matcher () -> Result < () > { let first_matcher = starts_with ("A") ; let second_matcher = ends_with ("string") ; let matcher = any ! (first_matcher , second_matcher) ; verify_that ! (Matcher ::<& String >:: describe (& matcher , MatcherResult :: Match) , displays_as (eq (indoc ! ("
                has at least one of the following properties:
                  * starts with prefix \"A\"
                  * ends with suffix \"string\"")))) } # [test] fn description_shows_one_matcher_directly () -> Result < () > { let first_matcher = starts_with ("A") ; let matcher = any ! (first_matcher) ; verify_that ! (Matcher ::<& str >:: describe (& matcher , MatcherResult :: Match) , displays_as (eq ("starts with prefix \"A\""))) } # [test] fn mismatch_description_shows_which_matcher_failed_if_more_than_one_constituent () -> Result < () > { let first_matcher = starts_with ("Another") ; let second_matcher = ends_with ("string") ; let matcher = any ! (first_matcher , second_matcher) ; verify_that ! (matcher . explain_match ("A string") , displays_as (eq ("which does not start with \"Another\""))) } # [test] fn mismatch_description_is_simple_when_only_one_constituent () -> Result < () > { let first_matcher = starts_with ("Another") ; let matcher = any ! (first_matcher) ; verify_that ! (matcher . explain_match ("A string") , displays_as (eq ("which does not start with \"Another\""))) } # [test] fn empty_any_matcher_never_matches () -> Result < () > { verify_that ! (123 , not (any ! [])) } # [test] fn any_with_auto_eq () -> Result < () > { verify_that ! (42 , any ! [1 , 2 , 42 , gt (123)]) } }
};
}
