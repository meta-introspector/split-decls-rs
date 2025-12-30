// Generated macro for unescape_unicode (function)
macro_rules! Depcrate_parserunescape_unicode {
() => {
// Module: crate::parser
// Provides: {"unescape_unicode"}
// Dependencies: {}
fn unescape_unicode (chars : & mut Chars) -> Option < (char , char) > { let mut value = 0 ; for (i , c) in chars . enumerate () { match (i , c . to_digit (16)) { (0 ..= 5 , Some (num)) => value = (value << 4) | num , (1 .. , None) => return Some ((char :: from_u32 (value) ? , c)) , _ => break , } } None }
};
}
