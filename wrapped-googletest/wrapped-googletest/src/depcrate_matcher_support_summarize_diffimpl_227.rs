// Generated macro for impl_227 (impl)
macro_rules! Depcrate_matcher_support_summarize_diffimpl_227 {
() => {
// Module: crate::matcher_support::summarize_diff
// Provides: {"impl_227"}
// Dependencies: {}
impl < 'a > FromIterator < edit_distance :: Edit < & 'a str > > for BufferedSummary < 'a > { fn from_iter < T : IntoIterator < Item = edit_distance :: Edit < & 'a str > > > (iter : T) -> Self { let mut buffered_summary = BufferedSummary :: default () ; for edit in iter { match edit { edit_distance :: Edit :: Both (same) => { buffered_summary . feed_common_lines (same) ; } edit_distance :: Edit :: ExtraActual (actual) => { buffered_summary . feed_extra_actual (actual) ; } edit_distance :: Edit :: ExtraExpected (expected) => { buffered_summary . feed_extra_expected (expected) ; } edit_distance :: Edit :: AdditionalActual => { buffered_summary . feed_additional_actual () ; } } ; } buffered_summary . flush_buffer () ; buffered_summary . summary . reset_ansi () ; buffered_summary } }
};
}
