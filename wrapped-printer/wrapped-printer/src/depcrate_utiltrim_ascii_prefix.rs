// Generated macro for trim_ascii_prefix (function)
macro_rules! Depcrate_utiltrim_ascii_prefix {
() => {
// Module: crate::util
// Provides: {"trim_ascii_prefix"}
// Dependencies: {}
# [doc = " Trim prefix ASCII spaces from the given slice and return the corresponding"] # [doc = " range."] # [doc = ""] # [doc = " This stops trimming a prefix as soon as it sees non-whitespace or a line"] # [doc = " terminator."] pub (crate) fn trim_ascii_prefix (line_term : LineTerminator , slice : & [u8] , range : Match ,) -> Match { fn is_space (b : u8) -> bool { match b { b'\t' | b'\n' | b'\x0B' | b'\x0C' | b'\r' | b' ' => true , _ => false , } } let count = slice [range] . iter () . take_while (| & & b | -> bool { is_space (b) && ! line_term . as_bytes () . contains (& b) }) . count () ; range . with_start (range . start () + count) }
};
}
