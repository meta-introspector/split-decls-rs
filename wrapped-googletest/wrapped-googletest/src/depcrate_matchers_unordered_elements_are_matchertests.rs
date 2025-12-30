// Generated macro for tests (module)
macro_rules! Depcrate_matchers_unordered_elements_are_matchertests {
() => {
// Module: crate::matchers::unordered_elements_are_matcher
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate as googletest ; use crate :: matcher :: MatcherResult ; use crate :: prelude :: * ; use indoc :: indoc ; use std :: collections :: HashMap ; # [test] fn has_correct_description_for_map () -> googletest :: Result < () > { let matchers = ((eq (& 2) , eq (& "Two")) , (eq (& 1) , eq (& "One")) , (eq (& 3) , eq (& "Three"))) ; let matcher = unordered_elements_are ! [(matchers . 0 . 0 , matchers . 0 . 1) , (matchers . 1 . 0 , matchers . 1 . 1) , (matchers . 2 . 0 , matchers . 2 . 1)] ; verify_that ! (Matcher ::<& HashMap < i32 , String >>:: describe (& matcher , MatcherResult :: Match) , displays_as (eq (indoc ! ("
                contains elements matching in any order:
                  0. is a tuple whose values respectively match:
                       is equal to 2
                       is equal to \"Two\"
                  1. is a tuple whose values respectively match:
                       is equal to 1
                       is equal to \"One\"
                  2. is a tuple whose values respectively match:
                       is equal to 3
                       is equal to \"Three\"")))) } # [test] fn unordered_elements_are_description_no_full_match_with_map () -> googletest :: Result < () > { let value : HashMap < u32 , u32 > = HashMap :: from_iter ([(0 , 1) , (1 , 1) , (2 , 2)]) ; let matchers = ((anything () , eq (& 1)) , (anything () , eq (& 2)) , (anything () , eq (& 2))) ; let matcher = unordered_elements_are ! [(matchers . 0 . 0 , matchers . 0 . 1) , (matchers . 1 . 0 , matchers . 1 . 1) , (matchers . 2 . 0 , matchers . 2 . 1) ,] ; verify_that ! (matcher . explain_match (& value) , all ! [displays_as (contains_regex ("Actual element \\(2, 2\\) at index [0-2] matched expected element `is a tuple whose values respectively match:\n    is anything\n    is equal to 2` at index [0-2].")) , displays_as (contains_regex ("Actual element \\(\n      [0-1],\n      [0-1],\n  \\) at index [0-2] did not match any remaining expected element.")) , displays_as (contains_substring ("Expected element `is a tuple whose values respectively match:\n    is anything\n    is equal to 2` at index 2 did not match any remaining actual element."))]) } }
};
}
