// Generated macro for with_leading_whitespace_inner (function)
macro_rules! Depcrate_sourcewith_leading_whitespace_inner {
() => {
// Module: crate::source
// Provides: {"with_leading_whitespace_inner"}
// Dependencies: {}
fn with_leading_whitespace_inner (lines : & [RelativeBytePos] , src : & str , range : Range < usize >) -> Option < usize > { debug_assert ! (lines . is_empty () || lines [0] . to_u32 () == 0) ; let start = src . get (.. range . start) ? . trim_end () ; let next_line = lines . partition_point (| & pos | pos . to_usize () <= start . len ()) ; if let Some (line_end) = lines . get (next_line) && line_end . to_usize () <= range . start && let prev_start = lines . get (next_line - 1) . map_or (0 , | & x | x . to_usize ()) && ends_with_line_comment_or_broken (& start [prev_start ..]) && let next_line = lines . partition_point (| & pos | pos . to_usize () < range . end) && let next_start = lines . get (next_line) . map_or (src . len () , | & x | x . to_usize ()) && tokenize (src . get (range . end .. next_start) ? , FrontmatterAllowed :: No) . any (| t | ! matches ! (t . kind , TokenKind :: Whitespace)) { Some (range . start) } else { Some (start . len ()) } }
};
}
