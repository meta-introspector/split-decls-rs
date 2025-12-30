// Generated macro for adjusted_arm_span (function)
macro_rules! Depcrate_matches_match_same_armsadjusted_arm_span {
() => {
// Module: crate::matches::match_same_arms
// Provides: {"adjusted_arm_span"}
// Dependencies: {}
# [doc = " Extend arm's span to include the comma and whitespaces after it."] fn adjusted_arm_span (cx : & LateContext < '_ > , span : Span) -> Span { let source_map = cx . sess () . source_map () ; source_map . span_extend_while (span , | c | c == ',' || c . is_ascii_whitespace ()) . unwrap_or (span) }
};
}
