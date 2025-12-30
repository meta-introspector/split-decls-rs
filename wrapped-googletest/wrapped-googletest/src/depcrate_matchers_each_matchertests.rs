// Generated macro for tests (module)
macro_rules! Depcrate_matchers_each_matchertests {
() => {
// Module: crate::matchers::each_matcher
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: prelude :: * ; use crate :: Result ; use indoc :: indoc ; use std :: collections :: HashSet ; # [test] fn each_matches_empty_vec () -> Result < () > { let value : Vec < i32 > = vec ! [] ; verify_that ! (value , each (gt (& 0))) } # [test] fn each_matches_vec_with_one_element () -> Result < () > { let value = vec ! [1] ; verify_that ! (value , each (gt (& 0))) } # [test] fn each_matches_vec_with_two_elements () -> Result < () > { let value = vec ! [1 , 2] ; verify_that ! (value , each (gt (& 0))) } # [test] fn each_matches_slice_with_one_element () -> Result < () > { let value = & [1] ; verify_that ! (* value , each (gt (0))) } # [test] fn each_matches_hash_set_with_one_element () -> Result < () > { let value : HashSet < i32 > = [1] . into () ; verify_that ! (value , each (gt (& 0))) } # [test] fn each_does_not_match_when_first_element_does_not_match () -> Result < () > { let value = vec ! [0] ; verify_that ! (value , not (each (gt (& 1)))) } # [test] fn each_does_not_match_when_second_element_does_not_match () -> Result < () > { let value = vec ! [2 , 0] ; verify_that ! (value , not (each (gt (& 1)))) } # [test] fn each_shows_correct_message_when_first_item_does_not_match () -> Result < () > { let result = verify_that ! (vec ! [0 , 2 , 3] , each (gt (& 0))) ; verify_that ! (result , err (displays_as (contains_substring (indoc ! ("
                Value of: vec![0, 2, 3]
                Expected: only contains elements that is greater than 0
                Actual: [0, 2, 3],
                  whose element #0 is 0, which is less than or equal to 0"))))) } # [test] fn each_shows_correct_message_when_second_item_does_not_match () -> Result < () > { let result = verify_that ! (vec ! [1 , 0 , 3] , each (gt (& 0))) ; verify_that ! (result , err (displays_as (contains_substring (indoc ! ("
                Value of: vec![1, 0, 3]
                Expected: only contains elements that is greater than 0
                Actual: [1, 0, 3],
                  whose element #1 is 0, which is less than or equal to 0"))))) } # [test] fn each_shows_correct_message_when_first_two_items_do_not_match () -> Result < () > { let result = verify_that ! (vec ! [0 , 1 , 3] , each (gt (& 1))) ; verify_that ! (result , err (displays_as (contains_substring (indoc ! ("
                Value of: vec![0, 1, 3]
                Expected: only contains elements that is greater than 1
                Actual: [0, 1, 3],
                  whose elements #0, #1 don't match
                    0, which is less than or equal to 1
                    1, which is less than or equal to 1"))))) } # [test] fn each_shows_inner_explanation () -> Result < () > { let result = verify_that ! (vec ! [vec ! [1 , 2] , vec ! [1]] , each (each (eq (& 1)))) ; verify_that ! (result , err (displays_as (contains_substring (indoc ! ("
                Expected: only contains elements that only contains elements that is equal to 1
                Actual: [[1, 2], [1]],
                  whose element #0 is [1, 2], whose element #1 is 2, which isn't equal to 1"))))) } }
};
}
