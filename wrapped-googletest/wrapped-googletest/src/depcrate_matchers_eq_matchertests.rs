// Generated macro for tests (module)
macro_rules! Depcrate_matchers_eq_matchertests {
() => {
// Module: crate::matchers::eq_matcher
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: prelude :: * ; use crate :: Result ; use indoc :: indoc ; # [test] fn eq_matches_string_reference_with_string_reference () -> Result < () > { verify_that ! ("A string" , eq ("A string")) } # [test] fn eq_matches_owned_string_with_string_reference () -> Result < () > { let value = "A string" . to_string () ; verify_that ! (value , eq ("A string")) } # [test] fn eq_matches_owned_string_reference_with_string_reference () -> Result < () > { let value = "A string" . to_string () ; verify_that ! (& value , eq ("A string")) } # [test] fn eq_matches_i32_with_i32 () -> Result < () > { verify_that ! (123 , eq (123)) } # [test] fn eq_struct_debug_diff () -> Result < () > { # [derive (Debug , PartialEq)] struct Strukt { int : i32 , string : String , } let result = verify_that ! (Strukt { int : 123 , string : "something" . into () } , eq (& Strukt { int : 321 , string : "someone" . into () })) ; verify_that ! (result , err (displays_as (contains_substring (indoc ! { "
            Actual: Strukt { int: 123, string: \"something\" },
              which isn't equal to Strukt { int: 321, string: \"someone\" }
              
              Difference(-actual / +expected):
               Strukt {
              -    int: 123,
              +    int: 321,
              -    string: \"something\",
              +    string: \"someone\",
               }
            " })))) } # [test] fn eq_vec_debug_diff () -> Result < () > { let result = verify_that ! (vec ! [1 , 2 , 3] , eq (& vec ! [1 , 3 , 4])) ; verify_that ! (result , err (displays_as (contains_substring (indoc ! { "
            Value of: vec![1, 2, 3]
            Expected: is equal to [1, 3, 4]
            Actual: [1, 2, 3],
              which isn't equal to [1, 3, 4]
              
              Difference(-actual / +expected):
               [
                   1,
              -    2,
                   3,
              +    4,
               ]
            " })))) } # [test] fn eq_vec_debug_diff_length_mismatch () -> Result < () > { let result = verify_that ! (vec ! [1 , 2 , 3 , 4 , 5] , eq (& vec ! [1 , 3 , 5])) ; verify_that ! (result , err (displays_as (contains_substring (indoc ! { "
            Value of: vec![1, 2, 3, 4, 5]
            Expected: is equal to [1, 3, 5]
            Actual: [1, 2, 3, 4, 5],
              which isn't equal to [1, 3, 5]
              
              Difference(-actual / +expected):
               [
                   1,
              -    2,
                   3,
              -    4,
                   5,
               ]
            " })))) } # [test] fn eq_debug_diff_common_lines_omitted () -> Result < () > { let result = verify_that ! ((1 .. 50) . collect ::< Vec < _ >> () , eq (& (3 .. 52) . collect ::< Vec < _ >> ())) ; verify_that ! (result , err (displays_as (contains_substring (indoc ! { "
            ],
              which isn't equal to [3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51]
              
              Difference(-actual / +expected):
               [
              -    1,
              -    2,
                   3,
                   4,
               <---- 43 common lines omitted ---->
                   48,
                   49,
              +    50,
              +    51,
               ]" })))) } # [test] fn eq_debug_diff_5_common_lines_not_omitted () -> Result < () > { let result = verify_that ! ((1 .. 8) . collect ::< Vec < _ >> () , eq (& (3 .. 10) . collect ::< Vec < _ >> ())) ; verify_that ! (result , err (displays_as (contains_substring (indoc ! { "
            Actual: [1, 2, 3, 4, 5, 6, 7],
              which isn't equal to [3, 4, 5, 6, 7, 8, 9]
              
              Difference(-actual / +expected):
               [
              -    1,
              -    2,
                   3,
                   4,
                   5,
                   6,
                   7,
              +    8,
              +    9,
               ]" })))) } # [test] fn eq_debug_diff_start_common_lines_omitted () -> Result < () > { let result = verify_that ! ((1 .. 50) . collect ::< Vec < _ >> () , eq (& (1 .. 52) . collect ::< Vec < _ >> ())) ; verify_that ! (result , err (displays_as (contains_substring (indoc ! { "
            ],
              which isn't equal to [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51]
              
              Difference(-actual / +expected):
               [
                   1,
               <---- 46 common lines omitted ---->
                   48,
                   49,
              +    50,
              +    51,
               ]" })))) } # [test] fn eq_debug_diff_end_common_lines_omitted () -> Result < () > { let result = verify_that ! ((1 .. 52) . collect ::< Vec < _ >> () , eq (& (3 .. 52) . collect ::< Vec < _ >> ())) ; verify_that ! (result , err (displays_as (contains_substring (indoc ! { "
            ],
              which isn't equal to [3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51]
              
              Difference(-actual / +expected):
               [
              -    1,
              -    2,
                   3,
                   4,
               <---- 46 common lines omitted ---->
                   51,
               ]" })))) } # [test] fn eq_multi_line_string_debug_diff () -> Result < () > { let result = verify_that ! ("One\nTwo\nThree" , eq ("One\nSix\nThree")) ; verify_that ! (result , err (displays_as (contains_substring (indoc ! { r#"
            Value of: "One\nTwo\nThree"
            Expected: is equal to "One\nSix\nThree"
            Actual: "One\nTwo\nThree",
              which isn't equal to "One\nSix\nThree"
            "# })))) } # [test] fn match_explanation_contains_diff_of_strings_if_more_than_one_line () -> Result < () > { let result = verify_that ! (indoc ! ("
                    First line
                    Second line
                    Third line
                ") , eq (indoc ! ("
                    First line
                    Second lines
                    Third line
                "))) ; verify_that ! (result , err (displays_as (contains_substring ("\
   First line
  -Second line
  +Second lines
   Third line")))) } # [test] fn match_explanation_does_not_show_diff_if_actual_value_is_single_line () -> Result < () > { let result = verify_that ! ("First line" , eq (indoc ! ("
                    First line
                    Second line
                    Third line
                "))) ; verify_that ! (result , err (displays_as (not (contains_substring ("Difference(-actual / +expected):"))))) } # [test] fn match_explanation_does_not_show_diff_if_expected_value_is_single_line () -> Result < () > { let result = verify_that ! (indoc ! ("
                    First line
                    Second line
                    Third line
                ") , eq ("First line")) ; verify_that ! (result , err (displays_as (not (contains_substring ("Difference(-actual / +expected):"))))) } }
};
}
