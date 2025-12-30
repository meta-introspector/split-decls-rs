// Generated macro for tests (module)
macro_rules! Depcrate_matchers_anything_matchertests {
() => {
// Module: crate::matchers::anything_matcher
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: prelude :: * ; use crate :: Result ; # [test] fn anything_matches_i32 () -> Result < () > { let value = 32 ; verify_that ! (value , anything ()) ? ; Ok (()) } # [test] fn anything_matches_str () -> Result < () > { let value = "32" ; verify_that ! (value , anything ()) ? ; Ok (()) } # [test] fn anything_matches_option () -> Result < () > { let value = Some (32) ; verify_that ! (value , some (anything ())) ? ; Ok (()) } }
};
}
