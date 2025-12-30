// Generated macro for tests (module)
macro_rules! Depcrate_matchers_container_eq_matchertests {
() => {
// Module: crate::matchers::container_eq_matcher
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: matcher :: MatcherResult ; use crate :: prelude :: * ; use crate :: Result ; use indoc :: indoc ; use std :: collections :: HashSet ; # [test] fn container_eq_returns_match_when_containers_match () -> Result < () > { verify_that ! (vec ! [1 , 2 , 3] , container_eq (vec ! [1 , 2 , 3])) } # [test] fn container_eq_matches_array_with_slice () -> Result < () > { let value = & [1 , 2 , 3] ; verify_that ! (value , container_eq ([1 , 2 , 3])) } # [test] fn container_eq_matches_hash_set () -> Result < () > { let value : HashSet < i32 > = [1 , 2 , 3] . into () ; verify_that ! (value , container_eq ([1 , 2 , 3] . into ())) } # [test] fn container_eq_full_error_message () -> Result < () > { let result = verify_that ! (vec ! [1 , 3 , 2] , container_eq (vec ! [1 , 2 , 3])) ; verify_that ! (result , err (displays_as (contains_substring (indoc ! ("
                    Value of: vec![1, 3, 2]
                    Expected: is equal to [1, 2, 3]
                    Actual: [1, 3, 2],
                      which contains all the elements
                "))))) } # [test] fn container_eq_returns_mismatch_when_elements_out_of_order () -> Result < () > { verify_that ! (container_eq (vec ! [1 , 2 , 3]) . explain_match (& vec ! [1 , 3 , 2]) , displays_as (eq ("which contains all the elements"))) } # [test] fn container_eq_mismatch_shows_missing_elements_in_container () -> Result < () > { verify_that ! (container_eq (vec ! [1 , 2 , 3]) . explain_match (& vec ! [1 , 2]) , displays_as (eq ("which is missing the element 3"))) } # [test] fn container_eq_mismatch_shows_surplus_elements_in_container () -> Result < () > { verify_that ! (container_eq (vec ! [1 , 2]) . explain_match (& vec ! [1 , 2 , 3]) , displays_as (eq ("which contains the unexpected element 3"))) } # [test] fn container_eq_mismatch_shows_missing_and_surplus_elements_in_container () -> Result < () > { verify_that ! (container_eq (vec ! [1 , 2 , 3]) . explain_match (& vec ! [1 , 2 , 4]) , displays_as (eq ("which is missing the element 3 and contains the unexpected element 4"))) } # [test] fn container_eq_mismatch_does_not_show_duplicated_element () -> Result < () > { verify_that ! (container_eq (vec ! [1 , 2 , 3]) . explain_match (& vec ! [1 , 2 , 3 , 3]) , displays_as (eq ("which contains all the elements"))) } # [test] fn container_eq_matches_owned_vec_with_array () -> Result < () > { let vector = vec ! [123 , 234] ; verify_that ! (vector , container_eq ([123 , 234])) } # [test] fn container_eq_matches_owned_vec_of_owned_strings_with_slice_of_string_references () -> Result < () > { let vector = vec ! ["A string" . to_string () , "Another string" . to_string ()] ; verify_that ! (vector , container_eq (["A string" , "Another string"])) } # [test] fn container_eq_matches_owned_vec_of_owned_strings_with_shorter_slice_of_string_references () -> Result < () > { let actual = vec ! ["A string" . to_string () , "Another string" . to_string ()] ; let matcher = container_eq (["A string"]) ; let result = matcher . matches (& actual) ; verify_that ! (result , eq (MatcherResult :: NoMatch)) } # [test] fn container_eq_mismatch_with_slice_shows_missing_elements_in_container () -> Result < () > { verify_that ! (container_eq ([1 , 2 , 3]) . explain_match (& vec ! [1 , 2]) , displays_as (eq ("which is missing the element 3"))) } # [test] fn container_eq_mismatch_with_str_slice_shows_missing_elements_in_container () -> Result < () > { verify_that ! (container_eq (["A" , "B" , "C"]) . explain_match (& vec ! ["A" . to_string () , "B" . to_string ()]) , displays_as (eq ("which is missing the element \"C\""))) } # [test] fn container_eq_mismatch_with_str_slice_shows_surplus_elements_in_container () -> Result < () > { verify_that ! (container_eq (["A" , "B"]) . explain_match (& vec ! ["A" . to_string () , "B" . to_string () , "C" . to_string ()]) , displays_as (eq ("which contains the unexpected element \"C\""))) } # [test] fn ignoring_order_match () -> Result < () > { verify_that ! (vec ! ["a" , "b"] , container_eq (["b" , "a"]) . ignore_order ()) } # [test] fn ignoring_order_mismatch () -> Result < () > { verify_that ! (vec ! ["a" , "b"] , not (container_eq (["1" , "2"]) . ignore_order ())) } # [test] fn ignoring_order_mismatch_explain () -> Result < () > { let expected_err = verify_that ! (vec ! ["a" , "b"] , container_eq (["1" , "2"]) . ignore_order ()) ; verify_that ! (expected_err , err (displays_as (contains_substring (indoc ! (r#"
                Value of: vec!["a", "b"]
                Expected: contains all elements matching in any order:
                  * "1"
                  * "2"
                Actual: ["a", "b"],
                  which does not have a perfect match.  The best match found was:
                    Actual element "a" at index 0 did not match any remaining expected element.
                    Actual element "b" at index 1 did not match any remaining expected element.
                    Expected element "1" at index 0 did not match any remaining actual element.
                    Expected element "2" at index 1 did not match any remaining actual element.
                "#))))) } # [test] fn ignoring_order_unaccounted_extra_expected () -> Result < () > { verify_that ! (vec ! ["a" , "b"] , not (container_eq (["a" , "b" , "a"]) . ignore_order ())) } # [test] fn ignoring_order_unaccounted_extra_expected_explain () -> Result < () > { let expected_err = verify_that ! (vec ! ["a" , "b"] , container_eq (["a" , "b" , "a"]) . ignore_order ()) ; verify_that ! (expected_err , err (displays_as (contains_substring (indoc ! (r#"
                Value of: vec!["a", "b"]
                Expected: contains all elements matching in any order:
                  * "a"
                  * "b"
                  * "a"
                Actual: ["a", "b"],
                  which does not have a perfect match.  The best match found was:
                    Actual element "a" at index 0 is equal to expected element at index 0.
                    Actual element "b" at index 1 is equal to expected element at index 1.
                    Expected element "a" at index 2 did not match any remaining actual element.
                "#))))) } # [test] fn ignoring_order_unaccounted_extra_actual () -> Result < () > { verify_that ! (vec ! ["a" , "b" , "a"] , not (container_eq (["b" , "a"]) . ignore_order ())) } # [test] fn ignoring_order_unaccounted_extra_actual_explain () -> Result < () > { let expected_err = verify_that ! (vec ! ["a" , "b" , "a"] , container_eq (["b" , "a"]) . ignore_order ()) ; verify_that ! (expected_err , err (displays_as (contains_substring (indoc ! (r#"
                Value of: vec!["a", "b", "a"]
                Expected: contains all elements matching in any order:
                  * "b"
                  * "a"
                Actual: ["a", "b", "a"],
                  which does not have a perfect match.  The best match found was:
                    Actual element "a" at index 0 is equal to expected element at index 1.
                    Actual element "b" at index 1 is equal to expected element at index 0.
                    Actual element "a" at index 2 did not match any remaining expected element.
                "#))))) } # [test] fn ignoring_order_on_sets () -> Result < () > { let mut actual = std :: collections :: HashSet :: new () ; actual . insert ("b") ; actual . insert ("a") ; actual . insert ("c") ; verify_that ! (actual , container_eq (["c" , "b" , "a"]) . ignore_order ()) } # [test] fn ignoring_order_on_sets_explain () -> Result < () > { let mut actual = std :: collections :: HashSet :: new () ; actual . insert ("b") ; actual . insert ("a") ; actual . insert ("c") ; let expected_err = verify_that ! (actual , container_eq (["c" , "a"]) . ignore_order ()) ; verify_that ! (expected_err , err (displays_as (contains_regex (indoc ! (r#"
                Value of: actual
                Expected: contains all elements matching in any order:
                  \* "c"
                  \* "a"
                Actual: \{"\w", "\w", "\w"\},
                  which does not have a perfect match.  The best match found was:
                    Actual element "\w" at index \d is equal to expected element at index \d\.
                    Actual element "\w" at index \d is equal to expected element at index \d\.
                    Actual element "\w" at index \d did not match any remaining expected element\.
                "#))))) } # [test] fn ignoring_order_on_number_sets () -> Result < () > { let mut actual = std :: collections :: HashSet :: new () ; actual . insert (1) ; actual . insert (2) ; actual . insert (3) ; verify_that ! (actual , container_eq ([3 , 2 , 1]) . ignore_order ()) } }
};
}
