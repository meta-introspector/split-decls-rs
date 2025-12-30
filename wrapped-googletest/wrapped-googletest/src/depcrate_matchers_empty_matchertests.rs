// Generated macro for tests (module)
macro_rules! Depcrate_matchers_empty_matchertests {
() => {
// Module: crate::matchers::empty_matcher
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: prelude :: * ; use crate :: Result ; use std :: collections :: HashSet ; # [test] fn empty_matcher_match_empty_vec () -> Result < () > { let value : Vec < i32 > = vec ! [] ; verify_that ! (value , is_empty ()) } # [test] fn empty_matcher_does_not_match_empty_vec () -> Result < () > { let value = vec ! [1 , 2 , 3] ; verify_that ! (value , not (is_empty ())) } # [test] fn empty_matcher_matches_empty_slice () -> Result < () > { let value : & [i32] = & [] ; verify_that ! (value , is_empty ()) } # [test] fn empty_matcher_matches_empty_hash_set () -> Result < () > { let value : HashSet < i32 > = HashSet :: new () ; verify_that ! (value , is_empty ()) } }
};
}
