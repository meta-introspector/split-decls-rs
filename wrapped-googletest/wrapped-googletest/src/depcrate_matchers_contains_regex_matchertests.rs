// Generated macro for tests (module)
macro_rules! Depcrate_matchers_contains_regex_matchertests {
() => {
// Module: crate::matchers::contains_regex_matcher
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: matcher :: MatcherResult ; use crate :: prelude :: * ; use crate :: Result ; # [test] fn contains_regex_matches_string_reference_with_pattern () -> Result < () > { let matcher = contains_regex ("S.*val") ; let result = matcher . matches ("Some value") ; verify_that ! (result , eq (MatcherResult :: Match)) } # [test] fn contains_regex_does_not_match_string_without_pattern () -> Result < () > { let matcher = contains_regex ("Another") ; let result = matcher . matches ("Some value") ; verify_that ! (result , eq (MatcherResult :: NoMatch)) } # [test] fn contains_regex_matches_owned_string_with_pattern () -> Result < () > { let matcher = contains_regex ("value") ; let result = matcher . matches (& "Some value" . to_string ()) ; verify_that ! (result , eq (MatcherResult :: Match)) } # [test] fn contains_regex_matches_string_reference_with_owned_string () -> Result < () > { let matcher = contains_regex ("value") ; let result = matcher . matches ("Some value") ; verify_that ! (result , eq (MatcherResult :: Match)) } # [test] fn verify_that_works_with_owned_string () -> Result < () > { verify_that ! ("Some value" . to_string () , contains_regex ("value")) } # [test] fn contains_regex_displays_quoted_debug_of_pattern () -> Result < () > { let matcher = contains_regex ("\n") ; verify_that ! (Matcher ::<& str >:: describe (& matcher , MatcherResult :: Match) , displays_as (eq ("contains the regular expression \"\\n\""))) } }
};
}
