// Generated macro for tests (module)
macro_rules! Depcrate_matchers_points_to_matchertests {
() => {
// Module: crate::matchers::points_to_matcher
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: prelude :: * ; use crate :: Result ; use indoc :: indoc ; # [test] fn points_to_matches_ref () -> Result < () > { verify_that ! (& 123 , points_to (eq (123))) } # [test] fn match_explanation_references_actual_value () -> Result < () > { let result = verify_that ! (& 1 , points_to (eq (0))) ; verify_that ! (result , err (displays_as (contains_substring (indoc ! ("
                    Actual: 1,
                      which isn't equal to 0
                "))))) } }
};
}
