// Generated macro for split_suffix (function)
macro_rules! Depcrate_numeric_literalsplit_suffix {
() => {
// Module: crate::numeric_literal
// Provides: {"split_suffix"}
// Dependencies: {}
fn split_suffix < 'a > (src : & 'a str , lit_kind : & LitKind) -> (& 'a str , Option < & 'a str >) { debug_assert ! (lit_kind . is_numeric ()) ; lit_suffix_length (lit_kind) . and_then (| suffix_length | src . len () . checked_sub (suffix_length)) . map_or ((src , None) , | split_pos | { let (unsuffixed , suffix) = src . split_at (split_pos) ; (unsuffixed , Some (suffix)) }) }
};
}
