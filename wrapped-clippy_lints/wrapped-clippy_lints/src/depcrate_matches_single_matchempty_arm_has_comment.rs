// Generated macro for empty_arm_has_comment (function)
macro_rules! Depcrate_matches_single_matchempty_arm_has_comment {
() => {
// Module: crate::matches::single_match
// Provides: {"empty_arm_has_comment"}
// Dependencies: {}
# [doc = " Checks if there are comments contained within a span."] # [doc = " This is a very \"naive\" check, as it just looks for the literal characters // and /* in the"] # [doc = " source text. This won't be accurate if there are potentially expressions contained within the"] # [doc = " span, e.g. a string literal `\"//\"`, but we know that this isn't the case for empty"] # [doc = " match arms."] fn empty_arm_has_comment (cx : & LateContext < '_ > , span : Span) -> bool { span . check_source_text (cx , | text | text . as_bytes () . windows (2) . any (| w | w == b"//" || w == b"/*")) }
};
}
