// Generated macro for tests (module)
macro_rules! Depcrate_matchers_derefs_to_matchertests {
() => {
// Module: crate::matchers::derefs_to_matcher
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use std :: rc :: Rc ; use crate :: prelude :: * ; use crate :: Result ; use indoc :: indoc ; # [test] fn deref_to_matches_box_of_int_with_int () -> Result < () > { let actual = Box :: new (123) ; verify_that ! (actual , derefs_to (eq (& 123))) } # [test] fn deref_to_matches_rc_of_int_with_int () -> Result < () > { verify_that ! (Rc :: new (123) , derefs_to (eq (& 123))) } # [test] fn deref_to_combines_with_points_to_for_copy () -> Result < () > { verify_that ! (Rc :: new (123) , derefs_to (points_to (eq (123)))) } # [test] fn match_explanation_references_actual_value () -> Result < () > { let actual = Box :: new (1) ; let result = verify_that ! (actual , derefs_to (eq (& 0))) ; verify_that ! (result , err (displays_as (contains_substring (indoc ! ("
                    Actual: 1,
                      which isn't equal to 0
                "))))) } }
};
}
