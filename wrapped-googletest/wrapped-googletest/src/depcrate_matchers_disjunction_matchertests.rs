// Generated macro for tests (module)
macro_rules! Depcrate_matchers_disjunction_matchertests {
() => {
// Module: crate::matchers::disjunction_matcher
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: prelude :: * ; use crate :: Result ; use indoc :: indoc ; # [test] fn or_true_true_matches () -> Result < () > { verify_that ! (1 , anything () . or (anything ())) } # [test] fn or_true_false_matches () -> Result < () > { verify_that ! (1 , anything () . or (not (anything ()))) } # [test] fn or_false_true_matches () -> Result < () > { verify_that ! (1 , not (anything ()) . or (anything ())) } # [test] fn or_false_false_does_not_match () -> Result < () > { let result = verify_that ! (1 , not (anything ()) . or (not (anything ()))) ; verify_that ! (result , err (displays_as (contains_substring (indoc ! ("
                Value of: 1
                Expected: has at least one of the following properties:
                  * never matches
                  * never matches
                Actual: 1,
                  * which is anything
                  * which is anything
                "))))) } # [test] fn chained_or_matches () -> Result < () > { verify_that ! (10 , eq (1) . or (eq (5)) . or (ge (9))) } # [test] fn works_with_str_slices () -> Result < () > { verify_that ! ("A string" , ends_with ("A") . or (ends_with ("string"))) } # [test] fn works_with_owned_strings () -> Result < () > { verify_that ! ("A string" . to_string () , ends_with ("A") . or (ends_with ("string"))) } }
};
}
