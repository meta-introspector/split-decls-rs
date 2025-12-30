// Generated macro for tests (module)
macro_rules! Depcrate_matchers_conjunction_matchertests {
() => {
// Module: crate::matchers::conjunction_matcher
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: prelude :: * ; use crate :: Result ; use indoc :: indoc ; # [test] fn and_true_true_matches () -> Result < () > { verify_that ! (1 , anything () . and (anything ())) } # [test] fn and_true_false_does_not_match () -> Result < () > { let result = verify_that ! (1 , anything () . and (not (anything ()))) ; verify_that ! (result , err (displays_as (contains_substring (indoc ! ("
                Value of: 1
                Expected: has all the following properties:
                  * is anything
                  * never matches
                Actual: 1,
                  which is anything
                "))))) } # [test] fn and_false_true_does_not_match () -> Result < () > { let result = verify_that ! (1 , not (anything ()) . and (anything ())) ; verify_that ! (result , err (displays_as (contains_substring (indoc ! ("
                    Value of: 1
                    Expected: has all the following properties:
                      * never matches
                      * is anything
                    Actual: 1,
                      which is anything
                "))))) } # [test] fn and_false_false_does_not_match () -> Result < () > { let result = verify_that ! (1 , not (anything ()) . and (not (anything ()))) ; verify_that ! (result , err (displays_as (contains_substring (indoc ! ("
                Value of: 1
                Expected: has all the following properties:
                  * never matches
                  * never matches
                Actual: 1,
                  * which is anything
                  * which is anything
                "))))) } # [test] fn and_long_chain_of_matchers () -> Result < () > { let result = verify_that ! (1 , anything () . and (not (anything ())) . and (anything ()) . and (not (anything ())) . and (anything ())) ; verify_that ! (result , err (displays_as (contains_substring (indoc ! ("
                Value of: 1
                Expected: has all the following properties:
                  * is anything
                  * never matches
                  * is anything
                  * never matches
                  * is anything
                Actual: 1,
                  * which is anything
                  * which is anything
                "))))) } # [test] fn chained_and_matches () -> Result < () > { # [derive (Debug , Clone , Copy)] struct Struct { a : i32 , b : i32 , c : i32 , } verify_that ! (Struct { a : 1 , b : 2 , c : 3 } , field ! (Struct . a , eq (1)) . and (field ! (Struct . b , eq (2))) . and (field ! (Struct . c , eq (3)))) } # [test] fn works_with_str_slices () -> Result < () > { verify_that ! ("A string" , starts_with ("A") . and (ends_with ("string"))) } # [test] fn works_with_owned_strings () -> Result < () > { verify_that ! ("A string" . to_string () , starts_with ("A") . and (ends_with ("string"))) } }
};
}
