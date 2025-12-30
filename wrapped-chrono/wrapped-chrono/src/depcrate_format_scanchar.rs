// Generated macro for char (function)
macro_rules! Depcrate_format_scanchar {
() => {
// Module: crate::format::scan
// Provides: {"char"}
// Dependencies: {}
# [doc = " Tries to consume exactly one given character."] pub (super) fn char (s : & str , c1 : u8) -> ParseResult < & str > { match s . as_bytes () . first () { Some (& c) if c == c1 => Ok (& s [1 ..]) , Some (_) => Err (INVALID) , None => Err (TOO_SHORT) , } }
};
}
