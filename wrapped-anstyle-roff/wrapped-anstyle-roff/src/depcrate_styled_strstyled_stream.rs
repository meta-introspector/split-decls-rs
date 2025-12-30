// Generated macro for styled_stream (function)
macro_rules! Depcrate_styled_strstyled_stream {
() => {
// Module: crate::styled_str
// Provides: {"styled_stream"}
// Dependencies: {}
# [doc = " Produce a stream of [`StyledStr`] from text that contains ansi escape sequences"] pub (crate) fn styled_stream (text : & str) -> impl Iterator < Item = StyledStr < '_ > > { let categorized = cansi :: v3 :: categorise_text (text) ; categorized . into_iter () . map (| x | x . into ()) }
};
}
