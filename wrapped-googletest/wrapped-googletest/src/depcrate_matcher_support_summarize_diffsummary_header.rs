// Generated macro for summary_header (function)
macro_rules! Depcrate_matcher_support_summarize_diffsummary_header {
() => {
// Module: crate::matcher_support::summarize_diff
// Provides: {"summary_header"}
// Dependencies: {}
fn summary_header () -> Cow < 'static , str > { if USE_COLOR . with (Cell :: get) { format ! ("Difference(-{ACTUAL_ONLY_STYLE}actual{RESET_ALL} / +{EXPECTED_ONLY_STYLE}expected{RESET_ALL}):") . into () } else { "Difference(-actual / +expected):" . into () } }
};
}
