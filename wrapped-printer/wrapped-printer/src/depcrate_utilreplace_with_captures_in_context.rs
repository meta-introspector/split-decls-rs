// Generated macro for replace_with_captures_in_context (function)
macro_rules! Depcrate_utilreplace_with_captures_in_context {
() => {
// Module: crate::util
// Provides: {"replace_with_captures_in_context"}
// Dependencies: {}
# [doc = " Like `Matcher::replace_with_captures_at`, but accepts an end bound."] # [doc = ""] # [doc = " See also: `find_iter_at_in_context` for why we need this."] fn replace_with_captures_in_context < M , F > (matcher : M , bytes : & [u8] , line_terminator : & [u8] , range : std :: ops :: Range < usize > , caps : & mut M :: Captures , dst : & mut Vec < u8 > , mut append : F ,) -> Result < () , M :: Error > where M : Matcher , F : FnMut (& M :: Captures , & mut Vec < u8 >) -> bool , { let mut last_match = range . start ; matcher . captures_iter_at (bytes , range . start , caps , | caps | { let m = caps . get (0) . unwrap () ; if m . start () >= range . end { return false ; } dst . extend (& bytes [last_match .. m . start ()]) ; last_match = m . end () ; append (caps , dst) }) ? ; let end = if last_match > range . end { bytes . len () } else { std :: cmp :: min (bytes . len () , range . end) } ; dst . extend (& bytes [last_match .. end]) ; dst . extend (line_terminator) ; Ok (()) }
};
}
