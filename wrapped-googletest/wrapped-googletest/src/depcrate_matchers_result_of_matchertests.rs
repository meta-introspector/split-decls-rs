// Generated macro for tests (module)
macro_rules! Depcrate_matchers_result_of_matchertests {
() => {
// Module: crate::matchers::result_of_matcher
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: prelude :: * ; use crate :: Result ; use indoc :: indoc ; # [test] fn result_of_match_with_value () -> Result < () > { verify_that ! (1 , result_of ! (| value | value + 1 , eq (2))) } # [test] fn result_of_match_with_value_function () -> Result < () > { fn inc_by_one (value : i32) -> i32 { value + 1 } verify_that ! (1 , result_of ! (inc_by_one , eq (2))) } # [test] fn result_of_match_with_different_value () -> Result < () > { let result = verify_that ! (0 , result_of ! (| value | value - 1 , eq (2))) ; verify_that ! (result , err (displays_as (contains_substring (indoc ! ("
                Value of: 0
                Expected: by applying |value| value - 1,
                  is equal to 2
                Actual: 0,
                  which, results into -1
                    by applying |value| value - 1,
                      isn't equal to 2
                "))))) } # [test] fn result_of_match_with_different_value_block_closure () -> Result < () > { let result = verify_that ! (0 , result_of ! (| value | { value - 1 } , eq (2))) ; verify_that ! (result , err (displays_as (contains_substring (indoc ! ("
                Value of: 0
                Expected: by applying |value| { value - 1 },
                  is equal to 2
                Actual: 0,
                  which, results into -1
                    by applying |value| { value - 1 },
                      isn't equal to 2
                "))))) } # [test] fn result_of_match_with_different_value_multiline_closure () -> Result < () > { let result = verify_that ! (0 , result_of ! (| value | { let dec = value - 1 ; let inc = dec + 1 ; inc - 2 } , eq (2))) ; verify_that ! (result , err (displays_as (contains_substring (indoc ! ("
                Value of: 0
                Expected: by applying |value| { let dec = value - 1; let inc = dec + 1; inc - 2 },
                  is equal to 2
                Actual: 0,
                  which, results into -2
                    by applying |value| { let dec = value - 1; let inc = dec + 1; inc - 2 },
                      isn't equal to 2
                "))))) } # [test] fn result_of_match_with_different_value_function () -> Result < () > { fn dec_by_one (value : i32) -> i32 { value - 1 } let result = verify_that ! (0 , result_of ! (dec_by_one , eq (2))) ; verify_that ! (result , err (displays_as (contains_substring (indoc ! ("
                Value of: 0
                Expected: by applying dec_by_one,
                  is equal to 2
                Actual: 0,
                  which, results into -1
                    by applying dec_by_one,
                      isn't equal to 2
                "))))) } # [test] fn result_of_ref_match_with_string_reference () -> Result < () > { verify_that ! ("hello" , result_of_ref ! (| s : & str | s . to_uppercase () , eq ("HELLO"))) } # [test] fn result_of_ref_match_with_string_reference_function () -> Result < () > { fn to_upper_case < S : AsRef < str > > (s : S) -> String { s . as_ref () . to_uppercase () } verify_that ! ("hello" , result_of_ref ! (to_upper_case , eq ("HELLO"))) } # [test] fn result_of_ref_match_with_copy_types () -> Result < () > { verify_that ! (100 , result_of_ref ! (| value | value + 1 , eq (& 101))) } # [test] fn result_of_ref_match_with_different_value () -> Result < () > { let result = verify_that ! ("world" , result_of_ref ! (| s : & str | s . to_uppercase () , eq ("HELLO"))) ; verify_that ! (result , err (displays_as (contains_substring (indoc ! (r#"
                Value of: "world"
                Expected: by applying |s: &str| s.to_uppercase(),
                  is equal to "HELLO"
                Actual: "world",
                  which, results into "WORLD"
                    by applying |s: &str| s.to_uppercase(),
                      isn't equal to "HELLO""#))))) } # [test] fn result_of_ref_match_with_different_value_block_closure () -> Result < () > { let result = verify_that ! ("world" , result_of_ref ! (| s : & str | { s . to_uppercase () } , eq ("HELLO"))) ; verify_that ! (result , err (displays_as (contains_substring (indoc ! (r#"
            Value of: "world"
            Expected: by applying |s: &str| { s.to_uppercase() },
              is equal to "HELLO"
            Actual: "world",
              which, results into "WORLD"
                by applying |s: &str| { s.to_uppercase() },
                  isn't equal to "HELLO"
            "#))))) } # [test] fn result_of_ref_match_with_different_value_function () -> Result < () > { fn to_upper_case < S : AsRef < str > > (s : S) -> String { s . as_ref () . to_uppercase () } let result = verify_that ! ("world" , result_of_ref ! (to_upper_case , eq ("HELLO"))) ; verify_that ! (result , err (displays_as (contains_substring (indoc ! (r#"
            Value of: "world"
            Expected: by applying to_upper_case,
              is equal to "HELLO"
            Actual: "world",
              which, results into "WORLD"
                by applying to_upper_case,
                  isn't equal to "HELLO"
            "#))))) } # [test] fn result_of_ref_match_different_with_closure_variable () -> Result < () > { let to_upper_case = | s : & str | s . to_uppercase () ; let result = verify_that ! ("world" , result_of_ref ! (to_upper_case , eq ("HELLO"))) ; verify_that ! (result , err (displays_as (contains_substring (indoc ! (r#"
                Value of: "world"
                Expected: by applying to_upper_case,
                  is equal to "HELLO"
                Actual: "world",
                  which, results into "WORLD"
                    by applying to_upper_case,
                      isn't equal to "HELLO"
            "#))))) } # [test] fn result_of_ref_match_different_with_method_literal () -> Result < () > { let result = verify_that ! ("world" , result_of_ref ! (str :: to_uppercase , eq ("HELLO"))) ; verify_that ! (result , err (displays_as (contains_substring (indoc ! (r#"
                Value of: "world"
                Expected: by applying str::to_uppercase,
                  is equal to "HELLO"
                Actual: "world",
                  which, results into "WORLD"
                    by applying str::to_uppercase,
                      isn't equal to "HELLO"
            "#))))) } # [test] fn result_of_ref_match_different_with_function_return_closure () -> Result < () > { fn upper_case () -> impl Fn (& str) -> String { | s : & str | s . to_uppercase () } let result = verify_that ! ("world" , result_of_ref ! (upper_case () , eq ("HELLO"))) ; verify_that ! (result , err (displays_as (contains_substring (indoc ! (r#"
            Value of: "world"
            Expected: by applying upper_case(),
              is equal to "HELLO"
            Actual: "world",
              which, results into "WORLD"
                by applying upper_case(),
                  isn't equal to "HELLO"
            "#))))) } # [test] fn test_describe_simple () -> Result < () > { let matcher = result_of ! (| x | x + 1 , eq (2)) ; let description = matcher . describe (matcher . matches (0)) ; verify_that ! (description , displays_as (eq (indoc ! (r#"
        by applying |x| x + 1,
          isn't equal to 2"#)))) } # [test] fn test_describe_complicated () -> Result < () > { let matcher = result_of_ref ! (| s : & str | s . chars () . collect ::< Vec < _ >> () , each (predicate (char :: is_ascii_alphabetic))) ; let description = matcher . describe (matcher . matches ("A quick brown fox")) ; verify_that ! (description , displays_as (eq (indoc ! (r#"
        by applying |s: &str| s.chars().collect::<Vec<_>>(),
          contains no element that matches"#)))) } }
};
}
