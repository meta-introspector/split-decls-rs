// Generated macro for comment_2822 (function)
macro_rules! Depcrate_format_scancomment_2822 {
() => {
// Module: crate::format::scan
// Provides: {"comment_2822"}
// Dependencies: {}
# [doc = " Tries to consume an RFC2822 comment including preceding ` `."] # [doc = ""] # [doc = " Returns the remaining string after the closing parenthesis."] pub (super) fn comment_2822 (s : & str) -> ParseResult < (& str , ()) > { use CommentState :: * ; let s = s . trim_start () ; let mut state = Start ; for (i , c) in s . bytes () . enumerate () { state = match (state , c) { (Start , b'(') => Next (1) , (Next (1) , b')') => return Ok ((& s [i + 1 ..] , ())) , (Next (depth) , b'\\') => Escape (depth) , (Next (depth) , b'(') => Next (depth + 1) , (Next (depth) , b')') => Next (depth - 1) , (Next (depth) , _) | (Escape (depth) , _) => Next (depth) , _ => return Err (INVALID) , } ; } Err (TOO_SHORT) }
};
}
