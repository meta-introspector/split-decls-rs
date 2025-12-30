// Generated macro for find_iter_at_in_context (function)
macro_rules! Depcrate_utilfind_iter_at_in_context {
() => {
// Module: crate::util
// Provides: {"find_iter_at_in_context"}
// Dependencies: {}
pub (crate) fn find_iter_at_in_context < M , F > (searcher : & Searcher , matcher : M , mut bytes : & [u8] , range : std :: ops :: Range < usize > , mut matched : F ,) -> io :: Result < () > where M : Matcher , F : FnMut (Match) -> bool , { let is_multi_line = searcher . multi_line_with_matcher (& matcher) ; if is_multi_line { if bytes [range . end ..] . len () >= MAX_LOOK_AHEAD { bytes = & bytes [.. range . end + MAX_LOOK_AHEAD] ; } } else { let mut m = Match :: new (0 , range . end) ; trim_line_terminator (searcher , bytes , & mut m) ; bytes = & bytes [.. m . end ()] ; } matcher . find_iter_at (bytes , range . start , | m | { if m . start () >= range . end { return false ; } matched (m) }) . map_err (io :: Error :: error_message) }
};
}
