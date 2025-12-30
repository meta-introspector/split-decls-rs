// Generated macro for tests (module)
macro_rules! Depcrate_matchers_superset_of_matchertests {
() => {
// Module: crate::matchers::superset_of_matcher
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate as googletest ; use crate :: prelude :: * ; use indoc :: indoc ; use std :: collections :: HashSet ; # [test] fn superset_of_matches_empty_vec () -> googletest :: Result < () > { let value : Vec < i32 > = vec ! [] ; verify_that ! (value , superset_of ([])) } # [test] fn superset_of_matches_vec_with_one_element () -> googletest :: Result < () > { let value = vec ! [1] ; verify_that ! (value , superset_of ([& 1])) } # [test] fn superset_of_matches_vec_with_two_items () -> googletest :: Result < () > { let value = vec ! [1 , 2] ; verify_that ! (value , superset_of ([& 1 , & 2])) } # [test] fn superset_of_matches_vec_when_actual_has_excess_element () -> googletest :: Result < () > { let value = vec ! [1 , 2 , 3] ; verify_that ! (value , superset_of ([& 1 , & 2])) } # [test] fn superset_of_matches_vec_when_actual_has_excess_element_first () -> googletest :: Result < () > { let value = vec ! [3 , 1 , 2] ; verify_that ! (value , superset_of ([& 1 , & 2])) } # [test] fn superset_of_matches_slice_with_one_element () -> googletest :: Result < () > { let value = & [1] ; verify_that ! (value , superset_of ([& 1])) } # [test] fn superset_of_matches_hash_set_with_one_element () -> googletest :: Result < () > { let value : HashSet < i32 > = [1] . into () ; verify_that ! (value , superset_of ([& 1])) } # [test] fn superset_of_does_not_match_when_first_element_does_not_match () -> googletest :: Result < () > { let value = vec ! [0] ; verify_that ! (value , not (superset_of ([& 1]))) } # [test] fn superset_of_does_not_match_when_second_element_does_not_match () -> googletest :: Result < () > { let value = vec ! [2] ; verify_that ! (value , not (superset_of ([& 2 , & 0]))) } # [test] fn superset_of_shows_correct_message_when_first_item_does_not_match () -> googletest :: Result < () > { let result = verify_that ! (vec ! [0 , 2 , 3] , superset_of ([& 1 , & 2 , & 3])) ; verify_that ! (result , err (displays_as (contains_substring (indoc ! ("
                    Value of: vec![0, 2, 3]
                    Expected: is a superset of [
                        1,
                        2,
                        3,
                    ]
                    Actual: [0, 2, 3],
                      whose element 1 is missing
                "))))) } # [test] fn superset_of_shows_correct_message_when_second_item_does_not_match () -> googletest :: Result < () > { let result = verify_that ! (vec ! [1 , 0 , 3] , superset_of ([& 1 , & 2 , & 3])) ; verify_that ! (result , err (displays_as (contains_substring (indoc ! ("
                    Value of: vec![1, 0, 3]
                    Expected: is a superset of [
                        1,
                        2,
                        3,
                    ]
                    Actual: [1, 0, 3],
                      whose element 2 is missing
                "))))) } # [test] fn superset_of_shows_correct_message_when_first_two_items_do_not_match () -> googletest :: Result < () > { let result = verify_that ! (vec ! [0 , 0 , 3] , superset_of ([& 1 , & 2 , & 3])) ; verify_that ! (result , err (displays_as (contains_substring (indoc ! ("
                    Value of: vec![0, 0, 3]
                    Expected: is a superset of [
                        1,
                        2,
                        3,
                    ]
                    Actual: [0, 0, 3],
                      whose elements 1, 2 are missing
                "))))) } }
};
}
