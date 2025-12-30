// Generated macro for tests (module)
macro_rules! Depcrate_matchers_display_matchertests {
() => {
// Module: crate::matchers::display_matcher
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: prelude :: * ; use crate :: Result ; use indoc :: indoc ; use std :: fmt :: { Debug , Display , Error , Formatter } ; # [test] fn display_matches_i32 () -> Result < () > { let value = 32 ; verify_that ! (value , displays_as (eq ("32"))) ? ; Ok (()) } # [test] fn display_matches_str () -> Result < () > { let value = "32" ; verify_that ! (value , displays_as (eq ("32"))) ? ; Ok (()) } # [test] fn display_matches_struct () -> Result < () > { # [allow (dead_code)] # [derive (Debug)] struct Struct { a : i32 , b : i64 , } impl Display for Struct { fn fmt (& self , f : & mut Formatter < '_ >) -> std :: result :: Result < () , Error > { write ! (f , "{self:?}") } } verify_that ! (Struct { a : 123 , b : 321 } , displays_as (eq ("Struct { a: 123, b: 321 }"))) ? ; Ok (()) } # [test] fn display_displays_error_message_with_explanation_from_inner_matcher () -> Result < () > { let result = verify_that ! ("123\n234" , displays_as (eq ("123\n345"))) ; verify_that ! (result , err (displays_as (contains_substring (indoc ! ("
                  Actual: \"123\\n234\",
                    which displays as \"123\\n234\" which isn't equal to \"123\\n345\"
                    
                    Difference(-actual / +expected):
                     123
                    -234
                    +345
                "))))) } }
};
}
