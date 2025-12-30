// Generated macro for impl_230 (impl)
macro_rules! Depcrate_matcher_support_summarize_diffimpl_230 {
() => {
// Module: crate::matcher_support::summarize_diff
// Provides: {"impl_230"}
// Dependencies: {}
impl < 'a > Buffer < 'a > { fn flush (& mut self , summary : & mut SummaryBuilder) { match self { Buffer :: Empty => { } Buffer :: CommonLines (common_lines) => { Self :: flush_common_lines (std :: mem :: take (common_lines) , summary) ; } Buffer :: ExtraActualLineChunk (extra_actual) => { summary . new_line_for_actual () ; summary . push_str_actual_only (extra_actual) ; } Buffer :: ExtraExpectedLineChunk (extra_expected) => { summary . new_line_for_expected () ; summary . push_str_expected_only (extra_expected) ; } } ; * self = Buffer :: Empty ; } fn flush_common_lines (common_lines : Vec < & 'a str > , summary : & mut SummaryBuilder) { const COMMON_LINES_CONTEXT_SIZE : usize = 2 ; if common_lines . len () <= 2 * COMMON_LINES_CONTEXT_SIZE + 1 { for line in common_lines { summary . new_line () ; summary . push_str (line) ; } return ; } let start_context = & common_lines [0 .. COMMON_LINES_CONTEXT_SIZE] ; for line in start_context { summary . new_line () ; summary . push_str (line) ; } summary . new_line () ; summary . push_str_as_comment (& format ! ("<---- {} common lines omitted ---->" , common_lines . len () - 2 * COMMON_LINES_CONTEXT_SIZE ,)) ; let end_context = & common_lines [common_lines . len () - COMMON_LINES_CONTEXT_SIZE .. common_lines . len ()] ; for line in end_context { summary . new_line () ; summary . push_str (line) ; } } }
};
}
