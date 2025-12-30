// Generated macro for create_diff_reversed (function)
macro_rules! Depcrate_matcher_support_summarize_diffcreate_diff_reversed {
() => {
// Module: crate::matcher_support::summarize_diff
// Provides: {"create_diff_reversed"}
// Dependencies: {}
# [doc = " Returns a string describing how the expected and actual differ after"] # [doc = " reversing the lines in each."] # [doc = ""] # [doc = " This is similar to [`create_diff`] except that it first reverses the lines"] # [doc = " in both the expected and actual values, then reverses the constructed edit"] # [doc = " list. When `diff_mode` is [`edit_distance::Mode::Prefix`], this becomes a"] # [doc = " diff of the suffix for use by [`ends_with`][crate::matchers::ends_with]."] pub (crate) fn create_diff_reversed (actual_debug : & str , expected_debug : & str , diff_mode : edit_distance :: Mode ,) -> Cow < 'static , str > { if actual_debug . lines () . count () < 2 { return "" . into () ; } let mut actual_lines_reversed = actual_debug . lines () . collect :: < Vec < _ > > () ; let mut expected_lines_reversed = expected_debug . lines () . collect :: < Vec < _ > > () ; actual_lines_reversed . reverse () ; expected_lines_reversed . reverse () ; match edit_distance :: edit_list (actual_lines_reversed , expected_lines_reversed , diff_mode) { edit_distance :: Difference :: Equal => "No difference found between debug strings." . into () , edit_distance :: Difference :: Editable (mut edit_list) => { edit_list . reverse () ; format ! ("{}{}" , summary_header () , edit_list . into_iter () . collect ::< BufferedSummary > () ,) . into () } edit_distance :: Difference :: Unrelated => "" . into () , } }
};
}
