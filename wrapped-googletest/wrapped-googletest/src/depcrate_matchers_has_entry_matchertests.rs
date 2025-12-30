// Generated macro for tests (module)
macro_rules! Depcrate_matchers_has_entry_matchertests {
() => {
// Module: crate::matchers::has_entry_matcher
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: prelude :: * ; use crate :: Result ; use indoc :: indoc ; use std :: collections :: HashMap ; # [test] fn has_entry_does_not_match_empty_hash_map () -> Result < () > { let value : HashMap < i32 , i32 > = HashMap :: new () ; verify_that ! (& value , not (has_entry (0 , eq (& 0)))) } # [test] fn has_entry_matches_hash_map_with_value () -> Result < () > { let value : HashMap < i32 , i32 > = HashMap :: from ([(0 , 0)]) ; verify_that ! (& value , has_entry (0 , eq (& 0))) } # [test] fn has_entry_does_not_match_hash_map_with_wrong_value () -> Result < () > { let value : HashMap < i32 , i32 > = HashMap :: from ([(0 , 1)]) ; verify_that ! (& value , not (has_entry (0 , eq (& 0)))) } # [test] fn has_entry_does_not_match_hash_map_with_wrong_key () -> Result < () > { let value : HashMap < i32 , i32 > = HashMap :: from ([(1 , 0)]) ; verify_that ! (& value , not (has_entry (0 , eq (& 0)))) } # [test] fn has_entry_shows_correct_message_when_key_is_not_present () -> Result < () > { let result = verify_that ! (HashMap :: from ([(0 , 0)]) , has_entry (1 , eq (& 0))) ; verify_that ! (result , err (displays_as (contains_substring (indoc ! ("
                Value of: HashMap::from([(0, 0)])
                Expected: contains key 1, which value is equal to 0
                Actual: {0: 0},
                  which doesn't contain key 1
                "))))) } # [test] fn has_entry_shows_correct_message_when_key_has_non_matching_value () -> Result < () > { let result = verify_that ! (HashMap :: from ([(0 , 0)]) , has_entry (0 , eq (& 1))) ; verify_that ! (result , err (displays_as (contains_substring (indoc ! ("
                Value of: HashMap::from([(0, 0)])
                Expected: contains key 0, which value is equal to 1
                Actual: {0: 0},
                  which contains key 0, but is mapped to value 0, which isn't equal to 1
                "))))) } }
};
}
