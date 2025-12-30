// Generated macro for tests (module)
macro_rules! Depcrate_matchers_subset_of_matchertests {
() => {
// Module: crate::matchers::subset_of_matcher
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: prelude :: * ; use crate :: Result ; use indoc :: indoc ; use std :: collections :: HashSet ; # [test] fn subset_of_matches_empty_vec () -> Result < () > { let value : Vec < i32 > = vec ! [] ; verify_that ! (value , subset_of ([])) } # [test] fn subset_of_matches_vec_with_one_element () -> Result < () > { let value = vec ! [1] ; verify_that ! (value , subset_of ([& 1])) } # [test] fn subset_of_matches_vec_with_two_elements () -> Result < () > { let value = vec ! [1 , 2] ; verify_that ! (value , subset_of ([& 1 , & 2])) } # [test] fn subset_of_matches_vec_when_expected_has_excess_element () -> Result < () > { let value = vec ! [1 , 2] ; verify_that ! (value , subset_of ([& 1 , & 2 , & 3])) } # [test] fn subset_of_matches_vec_when_expected_has_excess_element_first () -> Result < () > { let value = vec ! [1 , 2] ; verify_that ! (value , subset_of ([& 3 , & 1 , & 2])) } # [test] fn subset_of_matches_slice_with_one_element () -> Result < () > { let value = & [1] ; verify_that ! (value , subset_of ([& 1])) } # [test] fn subset_of_matches_hash_set_with_one_element () -> Result < () > { let value : HashSet < i32 > = [1] . into () ; verify_that ! (value , subset_of ([& 1])) } # [test] fn subset_of_does_not_match_when_first_element_does_not_match () -> Result < () > { let value = vec ! [0] ; verify_that ! (value , not (subset_of ([& 1]))) } # [test] fn subset_of_does_not_match_when_second_element_does_not_match () -> Result < () > { let value = vec ! [2 , 0] ; verify_that ! (value , not (subset_of ([& 2]))) } # [test] fn subset_of_shows_correct_message_when_first_item_does_not_match () -> Result < () > { let result = verify_that ! (vec ! [0 , 2 , 3] , subset_of ([& 1 , & 2 , & 3])) ; verify_that ! (result , err (displays_as (contains_substring (indoc ! ("
                    Value of: vec![0, 2, 3]
                    Expected: is a subset of [
                        1,
                        2,
                        3,
                    ]
                    Actual: [0, 2, 3],
                      whose element 0 at #0 is unexpected
                "))))) } # [test] fn subset_of_shows_correct_message_when_second_item_does_not_match () -> Result < () > { let result = verify_that ! (vec ! [1 , 0 , 3] , subset_of ([& 1 , & 2 , & 3])) ; verify_that ! (result , err (displays_as (contains_substring (indoc ! ("
                    Value of: vec![1, 0, 3]
                    Expected: is a subset of [
                        1,
                        2,
                        3,
                    ]
                    Actual: [1, 0, 3],
                      whose element 0 at #1 is unexpected
                "))))) } # [test] fn subset_of_shows_correct_message_when_first_two_items_do_not_match () -> Result < () > { let result = verify_that ! (vec ! [0 , 0 , 3] , subset_of ([& 1 , & 2 , & 3])) ; verify_that ! (result , err (displays_as (contains_substring (indoc ! ("
                    Value of: vec![0, 0, 3]
                    Expected: is a subset of [
                        1,
                        2,
                        3,
                    ]
                    Actual: [0, 0, 3],
                      whose elements 0 at #0, 0 at #1 are unexpected
                "))))) } }
};
}
