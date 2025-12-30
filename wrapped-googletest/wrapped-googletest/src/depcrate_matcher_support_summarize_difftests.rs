// Generated macro for tests (module)
macro_rules! Depcrate_matcher_support_summarize_difftests {
() => {
// Module: crate::matcher_support::summarize_diff
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use crate :: { matcher_support :: edit_distance :: Mode , prelude :: * , Result } ; use indoc :: indoc ; use std :: fmt :: Write ; fn build_text < T : Display > (mut collection : impl Iterator < Item = T >) -> String { let mut text = String :: new () ; write ! (& mut text , "{}" , collection . next () . expect ("Provided collection without elements")) . unwrap () ; for item in collection { write ! (& mut text , "\n{item}") . unwrap () ; } text } # [test] fn create_diff_smaller_than_one_line () -> Result < () > { verify_that ! (create_diff ("One" , "Two" , Mode :: Exact) , eq ("")) } # [test] fn create_diff_exact_same () -> Result < () > { let expected = indoc ! { "
            One
            Two
            " } ; let actual = indoc ! { "
        One
        Two
        " } ; verify_that ! (create_diff (expected , actual , Mode :: Exact) , eq ("No difference found between debug strings.")) } # [test] fn create_diff_multiline_diff () -> Result < () > { let expected = indoc ! { "
            prefix
            Actual#1
            Actual#2
            Actual#3
            suffix" } ; let actual = indoc ! { "
            prefix
            Expected@one
            Expected@two
            suffix" } ; verify_that ! (create_diff (expected , actual , Mode :: Exact) , eq (indoc ! ("
                Difference(-actual / +expected):
                 prefix
                -Actual#1
                +Expected@one
                -Actual#2
                +Expected@two
                -Actual#3
                 suffix"))) } # [test] fn create_diff_exact_unrelated () -> Result < () > { verify_that ! (create_diff (& build_text (1 .. 500) , & build_text (501 .. 1000) , Mode :: Exact) , eq ("")) } # [test] fn create_diff_exact_small_difference () -> Result < () > { verify_that ! (create_diff (& build_text (1 .. 50) , & build_text (1 .. 51) , Mode :: Exact) , eq (indoc ! { "
                Difference(-actual / +expected):
                 1
                 2
                 <---- 45 common lines omitted ---->
                 48
                 49
                +50" })) } # [test] fn create_diff_exact_small_difference_with_color () -> Result < () > { USE_COLOR . with (| cell | cell . set (true)) ; verify_that ! (create_diff (& build_text (1 .. 50) , & build_text (1 .. 51) , Mode :: Exact) , eq (indoc ! { "
                Difference(-\x1B[1;31mactual\x1B[0m / +\x1B[1;32mexpected\x1B[0m):
                 1
                 2
                 \x1B[3m<---- 45 common lines omitted ---->\x1B[0m
                 48
                 49
                +\x1B[1;32m50\x1B[0m" })) } # [test] fn create_diff_exact_difference_with_inline_color () -> Result < () > { USE_COLOR . with (| cell | cell . set (true)) ; let actual = indoc ! ("There is a home in Nouvelle Orleans
            They say, it is the rising sons
            And it has been the ruin of many a po'boy") ; let expected = indoc ! ("There is a house way down in New Orleans
            They call the rising sun
            And it has been the ruin of many a poor boy") ; verify_that ! (create_diff (actual , expected , Mode :: Exact) , eq (indoc ! { "
                Difference(-\x1B[1;31mactual\x1B[0m / +\x1B[1;32mexpected\x1B[0m):
                -\x1B[31mThere is a ho\x1B[0m\x1B[1;31mm\x1B[0m\x1B[31me in N\x1B[0m\x1B[1;31mouv\x1B[0m\x1B[31me\x1B[0m\x1B[1;31mlle\x1B[0m\x1B[31m Orleans\x1B[0m
                +\x1B[32mThere is a ho\x1B[0m\x1B[1;32mus\x1B[0m\x1B[32me \x1B[0m\x1B[1;32mway down \x1B[0m\x1B[32min Ne\x1B[0m\x1B[1;32mw\x1B[0m\x1B[32m Orleans\x1B[0m
                -\x1B[31mThey \x1B[0m\x1B[1;31ms\x1B[0m\x1B[31ma\x1B[0m\x1B[1;31my,\x1B[0m\x1B[31m \x1B[0m\x1B[1;31mi\x1B[0m\x1B[31mt\x1B[0m\x1B[1;31m is t\x1B[0m\x1B[31mhe rising s\x1B[0m\x1B[1;31mo\x1B[0m\x1B[31mn\x1B[0m\x1B[1;31ms\x1B[0m
                +\x1B[32mThey \x1B[0m\x1B[1;32mc\x1B[0m\x1B[32ma\x1B[0m\x1B[1;32mll\x1B[0m\x1B[32m the rising s\x1B[0m\x1B[1;32mu\x1B[0m\x1B[32mn\x1B[0m
                -\x1B[31mAnd it has been the ruin of many a po\x1B[0m\x1B[1;31m'\x1B[0m\x1B[31mboy\x1B[0m
                +\x1B[32mAnd it has been the ruin of many a po\x1B[0m\x1B[1;32mor \x1B[0m\x1B[32mboy\x1B[0m" })) } # [test] fn create_diff_line_termination_diff () -> Result < () > { verify_that ! (create_diff ("1\n2\n3" , "1\n2\n3\n" , Mode :: Exact) , eq ("Actual omits a terminating newline that is present in expected.")) ? ; verify_that ! (create_diff ("1\n2\n3\n" , "1\n2\n3" , Mode :: Exact) , eq ("Actual includes a terminating newline that is absent from expected.")) ? ; verify_that ! (create_diff ("1\n2\n3\n" , "1\n2\n3\n" , Mode :: Exact) , eq ("No difference found between debug strings.")) } }
};
}
