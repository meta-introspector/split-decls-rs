// Generated macro for tests (module)
macro_rules! Depcrate_matchers_str_matchertests {
() => {
// Module: crate::matchers::str_matcher
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate as googletest ; use crate :: matcher :: MatcherResult ; use crate :: prelude :: * ; use indoc :: indoc ; # [test] fn matches_string_reference_with_equal_string_reference () -> googletest :: Result < () > { let matcher = StrMatcher :: with_default_config ("A string") ; verify_that ! ("A string" , matcher) } # [test] fn does_not_match_string_reference_with_non_equal_string_reference () -> googletest :: Result < () > { let matcher = StrMatcher :: with_default_config ("Another string") ; verify_that ! ("A string" , not (matcher)) } # [test] fn matches_owned_string_with_string_reference () -> googletest :: Result < () > { let matcher = StrMatcher :: with_default_config ("A string") ; let value = "A string" . to_string () ; verify_that ! (value , matcher) } # [test] fn matches_owned_string_reference_with_string_reference () -> googletest :: Result < () > { let matcher = StrMatcher :: with_default_config ("A string") ; let value = "A string" . to_string () ; verify_that ! (& value , matcher) } # [test] fn ignores_leading_whitespace_in_expected_when_requested () -> googletest :: Result < () > { let matcher = StrMatcher :: with_default_config (" \n\tA string") ; verify_that ! ("A string" , matcher . ignoring_leading_whitespace ()) } # [test] fn ignores_leading_whitespace_in_actual_when_requested () -> googletest :: Result < () > { let matcher = StrMatcher :: with_default_config ("A string") ; verify_that ! (" \n\tA string" , matcher . ignoring_leading_whitespace ()) } # [test] fn does_not_match_unequal_remaining_string_when_ignoring_leading_whitespace () -> googletest :: Result < () > { let matcher = StrMatcher :: with_default_config (" \n\tAnother string") ; verify_that ! ("A string" , not (matcher . ignoring_leading_whitespace ())) } # [test] fn remains_sensitive_to_trailing_whitespace_when_ignoring_leading_whitespace () -> googletest :: Result < () > { let matcher = StrMatcher :: with_default_config ("A string \n\t") ; verify_that ! ("A string" , not (matcher . ignoring_leading_whitespace ())) } # [test] fn ignores_trailing_whitespace_in_expected_when_requested () -> googletest :: Result < () > { let matcher = StrMatcher :: with_default_config ("A string \n\t") ; verify_that ! ("A string" , matcher . ignoring_trailing_whitespace ()) } # [test] fn ignores_trailing_whitespace_in_actual_when_requested () -> googletest :: Result < () > { let matcher = StrMatcher :: with_default_config ("A string") ; verify_that ! ("A string \n\t" , matcher . ignoring_trailing_whitespace ()) } # [test] fn does_not_match_unequal_remaining_string_when_ignoring_trailing_whitespace () -> googletest :: Result < () > { let matcher = StrMatcher :: with_default_config ("Another string \n\t") ; verify_that ! ("A string" , not (matcher . ignoring_trailing_whitespace ())) } # [test] fn remains_sensitive_to_leading_whitespace_when_ignoring_trailing_whitespace () -> googletest :: Result < () > { let matcher = StrMatcher :: with_default_config (" \n\tA string") ; verify_that ! ("A string" , not (matcher . ignoring_trailing_whitespace ())) } # [test] fn ignores_leading_and_trailing_whitespace_in_expected_when_requested () -> googletest :: Result < () > { let matcher = StrMatcher :: with_default_config (" \n\tA string \n\t") ; verify_that ! ("A string" , matcher . ignoring_outer_whitespace ()) } # [test] fn ignores_leading_and_trailing_whitespace_in_actual_when_requested () -> googletest :: Result < () > { let matcher = StrMatcher :: with_default_config ("A string") ; verify_that ! (" \n\tA string \n\t" , matcher . ignoring_outer_whitespace ()) } # [test] fn respects_ascii_case_by_default () -> googletest :: Result < () > { let matcher = StrMatcher :: with_default_config ("A string") ; verify_that ! ("A STRING" , not (matcher)) } # [test] fn ignores_ascii_case_when_requested () -> googletest :: Result < () > { let matcher = StrMatcher :: with_default_config ("A string") ; verify_that ! ("A STRING" , matcher . ignoring_ascii_case ()) } # [test] fn ignores_unicode_case_when_requested () -> googletest :: Result < () > { let matcher = StrMatcher :: with_default_config ("ὈΔΥΣΣΕΎΣ") ; verify_that ! ("ὀδυσσεύς" , matcher . ignoring_unicode_case ()) } # [test] fn allows_ignoring_leading_whitespace_from_eq () -> googletest :: Result < () > { verify_that ! ("A string" , eq (" \n\tA string") . ignoring_leading_whitespace ()) } # [test] fn allows_ignoring_trailing_whitespace_from_eq () -> googletest :: Result < () > { verify_that ! ("A string" , eq ("A string \n\t") . ignoring_trailing_whitespace ()) } # [test] fn allows_ignoring_outer_whitespace_from_eq () -> googletest :: Result < () > { verify_that ! ("A string" , eq (" \n\tA string \n\t") . ignoring_outer_whitespace ()) } # [test] fn allows_ignoring_ascii_case_from_eq () -> googletest :: Result < () > { verify_that ! ("A string" , eq ("A STRING") . ignoring_ascii_case ()) } # [test] fn allows_ignoring_unicode_case_from_eq () -> googletest :: Result < () > { verify_that ! ("ὈΔΥΣΣΕΎΣ" , eq ("ὀδυσσεύς") . ignoring_unicode_case ()) } # [test] fn unicode_case_sensitive_from_eq () -> googletest :: Result < () > { verify_that ! ("ὈΔΥΣΣΕΎΣ" , not (eq ("ὀδυσσεύς"))) } # [test] fn matches_string_containing_expected_value_in_contains_mode () -> googletest :: Result < () > { verify_that ! ("Some string" , contains_substring ("str")) } # [test] fn matches_string_containing_expected_value_in_contains_mode_while_ignoring_ascii_case () -> googletest :: Result < () > { verify_that ! ("Some string" , contains_substring ("STR") . ignoring_ascii_case ()) } # [test] fn matches_string_containing_expected_value_in_contains_mode_while_ignoring_unicode_case () -> googletest :: Result < () > { verify_that ! ("Some σpsilon" , contains_substring ("Σps") . ignoring_unicode_case ()) } # [test] fn contains_substring_matches_correct_number_of_substrings () -> googletest :: Result < () > { verify_that ! ("Some string" , contains_substring ("str") . times (eq (1))) } # [test] fn contains_substring_does_not_match_incorrect_number_of_substrings () -> googletest :: Result < () > { verify_that ! ("Some string\nSome string" , not (contains_substring ("string") . times (eq (1)))) } # [test] fn contains_substring_does_not_match_when_substrings_overlap () -> googletest :: Result < () > { verify_that ! ("ababab" , not (contains_substring ("abab") . times (eq (2)))) } # [test] fn starts_with_matches_string_reference_with_prefix () -> googletest :: Result < () > { verify_that ! ("Some value" , starts_with ("Some")) } # [test] fn starts_with_matches_string_reference_with_prefix_ignoring_ascii_case () -> googletest :: Result < () > { verify_that ! ("Some value" , starts_with ("SOME") . ignoring_ascii_case ()) } # [test] fn starts_with_does_not_match_wrong_prefix_ignoring_ascii_case () -> googletest :: Result < () > { verify_that ! ("Some value" , not (starts_with ("OTHER") . ignoring_ascii_case ())) } # [test] fn starts_with_does_not_match_short_string_ignoring_ascii_case () -> googletest :: Result < () > { verify_that ! ("Some" , not (starts_with ("OTHER") . ignoring_ascii_case ())) } # [test] fn starts_with_matches_string_reference_with_prefix_ignoring_unicode_case () -> googletest :: Result < () > { verify_that ! ("비밀 santa" , starts_with ("비밀") . ignoring_unicode_case ()) } # [test] fn starts_with_does_not_match_wrong_prefix_ignoring_unicode_case () -> googletest :: Result < () > { verify_that ! ("secret santa" , not (starts_with ("비밀") . ignoring_unicode_case ())) } # [test] fn starts_with_does_not_match_short_string_ignoring_unicode_case () -> googletest :: Result < () > { verify_that ! ("비밀" , not (starts_with ("秘密") . ignoring_unicode_case ())) } # [test] fn starts_with_does_not_match_string_without_prefix () -> googletest :: Result < () > { verify_that ! ("Some value" , not (starts_with ("Another"))) } # [test] fn starts_with_does_not_match_string_with_substring_not_at_beginning () -> googletest :: Result < () > { verify_that ! ("Some value" , not (starts_with ("value"))) } # [test] fn ends_with_matches_string_reference_with_suffix () -> googletest :: Result < () > { verify_that ! ("Some value" , ends_with ("value")) } # [test] fn ends_with_matches_string_reference_with_suffix_ignoring_ascii_case () -> googletest :: Result < () > { verify_that ! ("Some value" , ends_with ("VALUE") . ignoring_ascii_case ()) } # [test] fn ends_with_does_not_match_wrong_suffix_ignoring_ascii_case () -> googletest :: Result < () > { verify_that ! ("Some value" , not (ends_with ("OTHER") . ignoring_ascii_case ())) } # [test] fn ends_with_does_not_match_too_short_string_ignoring_ascii_case () -> googletest :: Result < () > { verify_that ! ("Some" , not (ends_with ("OTHER") . ignoring_ascii_case ())) } # [test] fn ends_with_does_not_match_string_without_suffix () -> googletest :: Result < () > { verify_that ! ("Some value" , not (ends_with ("other value"))) } # [test] fn ends_with_does_not_match_string_with_substring_not_at_end () -> googletest :: Result < () > { verify_that ! ("Some value" , not (ends_with ("Some"))) } # [test] fn ends_with_matches_string_reference_with_suffix_ignoring_unicode_case () -> googletest :: Result < () > { verify_that ! ("santa 비밀" , ends_with ("비밀") . ignoring_unicode_case ()) } # [test] fn ends_with_does_not_match_wrong_suffix_ignoring_unicode_case () -> googletest :: Result < () > { verify_that ! ("secret santa" , not (ends_with ("비밀") . ignoring_unicode_case ())) } # [test] fn ends_with_does_not_match_short_string_ignoring_unicode_case () -> googletest :: Result < () > { verify_that ! ("비밀" , not (ends_with ("秘密") . ignoring_unicode_case ())) } # [test] fn describes_itself_for_matching_result () -> googletest :: Result < () > { let matcher = StrMatcher :: with_default_config ("A string") ; verify_that ! (Matcher ::<& str >:: describe (& matcher , MatcherResult :: Match) , displays_as (eq ("is equal to \"A string\""))) } # [test] fn describes_itself_for_non_matching_result () -> googletest :: Result < () > { let matcher = StrMatcher :: with_default_config ("A string") ; verify_that ! (Matcher ::<& str >:: describe (& matcher , MatcherResult :: NoMatch) , displays_as (eq ("isn't equal to \"A string\""))) } # [test] fn describes_itself_for_matching_result_ignoring_leading_whitespace () -> googletest :: Result < () > { let matcher = StrMatcher :: with_default_config ("A string") . ignoring_leading_whitespace () ; verify_that ! (Matcher ::<& str >:: describe (& matcher , MatcherResult :: Match) , displays_as (eq ("is equal to \"A string\" (ignoring leading whitespace)"))) } # [test] fn describes_itself_for_non_matching_result_ignoring_leading_whitespace () -> googletest :: Result < () > { let matcher = StrMatcher :: with_default_config ("A string") . ignoring_leading_whitespace () ; verify_that ! (Matcher ::<& str >:: describe (& matcher , MatcherResult :: NoMatch) , displays_as (eq ("isn't equal to \"A string\" (ignoring leading whitespace)"))) } # [test] fn describes_itself_for_matching_result_ignoring_trailing_whitespace () -> googletest :: Result < () > { let matcher = StrMatcher :: with_default_config ("A string") . ignoring_trailing_whitespace () ; verify_that ! (Matcher ::<& str >:: describe (& matcher , MatcherResult :: Match) , displays_as (eq ("is equal to \"A string\" (ignoring trailing whitespace)"))) } # [test] fn describes_itself_for_matching_result_ignoring_leading_and_trailing_whitespace () -> googletest :: Result < () > { let matcher = StrMatcher :: with_default_config ("A string") . ignoring_outer_whitespace () ; verify_that ! (Matcher ::<& str >:: describe (& matcher , MatcherResult :: Match) , displays_as (eq ("is equal to \"A string\" (ignoring leading and trailing whitespace)"))) } # [test] fn describes_itself_for_matching_result_ignoring_ascii_case () -> googletest :: Result < () > { let matcher = StrMatcher :: with_default_config ("A string") . ignoring_ascii_case () ; verify_that ! (Matcher ::<& str >:: describe (& matcher , MatcherResult :: Match) , displays_as (eq ("is equal to \"A string\" (ignoring ASCII case)"))) } # [test] fn describes_itself_for_matching_result_ignoring_ascii_case_and_leading_whitespace () -> googletest :: Result < () > { let matcher = StrMatcher :: with_default_config ("A string") . ignoring_leading_whitespace () . ignoring_ascii_case () ; verify_that ! (Matcher ::<& str >:: describe (& matcher , MatcherResult :: Match) , displays_as (eq ("is equal to \"A string\" (ignoring leading whitespace, ignoring ASCII case)"))) } # [test] fn describes_itself_for_matching_result_in_contains_mode () -> googletest :: Result < () > { let matcher = contains_substring ("A string") ; verify_that ! (Matcher ::<& str >:: describe (& matcher , MatcherResult :: Match) , displays_as (eq ("contains a substring \"A string\""))) } # [test] fn describes_itself_for_non_matching_result_in_contains_mode () -> googletest :: Result < () > { let matcher = contains_substring ("A string") ; verify_that ! (Matcher ::<& str >:: describe (& matcher , MatcherResult :: NoMatch) , displays_as (eq ("does not contain a substring \"A string\""))) } # [test] fn describes_itself_with_count_number () -> googletest :: Result < () > { let matcher = contains_substring ("A string") . times (gt (2)) ; verify_that ! (Matcher ::<& str >:: describe (& matcher , MatcherResult :: Match) , displays_as (eq ("contains a substring \"A string\" (count is greater than 2)"))) } # [test] fn describes_itself_for_matching_result_in_starts_with_mode () -> googletest :: Result < () > { let matcher = starts_with ("A string") ; verify_that ! (Matcher ::<& str >:: describe (& matcher , MatcherResult :: Match) , displays_as (eq ("starts with prefix \"A string\""))) } # [test] fn describes_itself_for_non_matching_result_in_starts_with_mode () -> googletest :: Result < () > { let matcher = starts_with ("A string") ; verify_that ! (Matcher ::<& str >:: describe (& matcher , MatcherResult :: NoMatch) , displays_as (eq ("does not start with \"A string\""))) } # [test] fn describes_itself_for_matching_result_in_ends_with_mode () -> googletest :: Result < () > { let matcher = ends_with ("A string") ; verify_that ! (Matcher ::<& str >:: describe (& matcher , MatcherResult :: Match) , displays_as (eq ("ends with suffix \"A string\""))) } # [test] fn describes_itself_for_non_matching_result_in_ends_with_mode () -> googletest :: Result < () > { let matcher = ends_with ("A string") ; verify_that ! (Matcher ::<& str >:: describe (& matcher , MatcherResult :: NoMatch) , displays_as (eq ("does not end with \"A string\""))) } # [test] fn match_explanation_contains_diff_of_strings_if_more_than_one_line () -> googletest :: Result < () > { let result = verify_that ! (indoc ! ("
                    First line
                    Second line
                    Third line
                ") , starts_with (indoc ! ("
                    First line
                    Second lines
                    Third line
                "))) ; verify_that ! (result , err (displays_as (contains_substring ("\
   First line
  -Second line
  +Second lines
   Third line")))) } # [test] fn match_explanation_for_starts_with_ignores_trailing_lines_in_actual_string () -> googletest :: Result < () > { let result = verify_that ! (indoc ! ("
                    First line
                    Second line
                    Third line
                    Fourth line
                ") , starts_with (indoc ! ("
                    First line
                    Second lines
                    Third line
                "))) ; verify_that ! (result , err (displays_as (contains_substring ("
   First line
  -Second line
  +Second lines
   Third line
   <---- remaining lines omitted ---->")))) } # [test] fn match_explanation_for_starts_with_includes_both_versions_of_differing_last_line () -> googletest :: Result < () > { let result = verify_that ! (indoc ! ("
                    First line
                    Second line
                    Third line
                ") , starts_with (indoc ! ("
                    First line
                    Second lines
                "))) ; verify_that ! (result , err (displays_as (contains_substring ("\
   First line
  -Second line
  +Second lines
   <---- remaining lines omitted ---->")))) } # [test] fn match_explanation_for_ends_with_ignores_leading_lines_in_actual_string () -> googletest :: Result < () > { let result = verify_that ! (indoc ! ("
                    First line
                    Second line
                    Third line
                    Fourth line
                ") , ends_with (indoc ! ("
                    Second line
                    Third lines
                    Fourth line
                "))) ; verify_that ! (result , err (displays_as (contains_substring ("
  Difference(-actual / +expected):
   <---- remaining lines omitted ---->
   Second line
  -Third line
  +Third lines
   Fourth line")))) } # [test] fn match_explanation_for_contains_substring_ignores_outer_lines_in_actual_string () -> googletest :: Result < () > { let result = verify_that ! (indoc ! ("
                    First line
                    Second line
                    Third line
                    Fourth line
                    Fifth line
                ") , contains_substring (indoc ! ("
                    Second line
                    Third lines
                    Fourth line
                "))) ; verify_that ! (result , err (displays_as (contains_substring ("
  Difference(-actual / +expected):
   <---- remaining lines omitted ---->
   Second line
  -Third line
  +Third lines
   Fourth line
   <---- remaining lines omitted ---->")))) } # [test] fn match_explanation_for_contains_substring_shows_diff_when_first_and_last_line_are_incomplete () -> googletest :: Result < () > { let result = verify_that ! (indoc ! ("
                    First line
                    Second line
                    Third line
                    Fourth line
                    Fifth line
                ") , contains_substring (indoc ! ("
                    line
                    Third line
                    Foorth line
                    Fifth"))) ; verify_that ! (result , err (displays_as (contains_substring ("
  Difference(-actual / +expected):
   <---- remaining lines omitted ---->
  -Second line
  +line
   Third line
  -Fourth line
  +Foorth line
  -Fifth line
  +Fifth
   <---- remaining lines omitted ---->")))) } # [test] fn match_explanation_for_eq_does_not_ignore_trailing_lines_in_actual_string () -> googletest :: Result < () > { let result = verify_that ! (indoc ! ("
                    First line
                    Second line
                    Third line
                    Fourth line
                ") , eq (indoc ! ("
                    First line
                    Second lines
                    Third line
                "))) ; verify_that ! (result , err (displays_as (contains_substring ("\
   First line
  -Second line
  +Second lines
   Third line
  -Fourth line")))) } # [test] fn match_explanation_does_not_show_diff_if_actual_value_is_single_line () -> googletest :: Result < () > { let result = verify_that ! ("First line" , starts_with (indoc ! ("
                    Second line
                    Third line
                "))) ; verify_that ! (result , err (displays_as (not (contains_substring ("Difference(-actual / +expected):"))))) } # [test] fn match_explanation_does_not_show_diff_if_expected_value_is_single_line () -> googletest :: Result < () > { let result = verify_that ! (indoc ! ("
                    First line
                    Second line
                    Third line
                ") , starts_with ("Second line")) ; verify_that ! (result , err (displays_as (not (contains_substring ("Difference(-actual / +expected):"))))) } }
};
}
