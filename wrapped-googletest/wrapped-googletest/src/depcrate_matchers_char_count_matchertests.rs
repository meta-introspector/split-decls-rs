// Generated macro for tests (module)
macro_rules! Depcrate_matchers_char_count_matchertests {
() => {
// Module: crate::matchers::char_count_matcher
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: description :: Description ; use crate :: matcher :: MatcherResult ; use crate :: prelude :: * ; use crate :: Result ; use indoc :: indoc ; use std :: fmt :: Debug ; # [test] fn char_count_matches_string_slice () -> Result < () > { let value = "abcd" ; verify_that ! (value , char_count (eq (4))) } # [test] fn char_count_matches_owned_string () -> Result < () > { let value = String :: from ("abcd") ; verify_that ! (value , char_count (eq (4))) } # [test] fn char_count_counts_non_ascii_characters_correctly () -> Result < () > { let value = "äöüß" ; verify_that ! (value , char_count (eq (4))) } # [test] fn char_count_explains_match () -> Result < () > { # [derive (MatcherBase)] struct TestMatcher ; impl < T : Debug + Copy > Matcher < T > for TestMatcher { fn matches (& self , _ : T) -> MatcherResult { false . into () } fn describe (& self , _ : MatcherResult) -> Description { "called described" . into () } fn explain_match (& self , _ : T) -> Description { "called explain_match" . into () } } verify_that ! (char_count (TestMatcher) . explain_match ("A string") , displays_as (eq ("which has character count 8, called explain_match"))) } # [test] fn char_count_has_correct_failure_message () -> Result < () > { let result = verify_that ! ("äöüß" , char_count (eq (3))) ; verify_that ! (result , err (displays_as (contains_substring (indoc ! (r#"
                Value of: "äöüß"
                Expected: has character count, which is equal to 3
                Actual: "äöüß",
                  which has character count 4, which isn't equal to 3"#))))) } }
};
}
