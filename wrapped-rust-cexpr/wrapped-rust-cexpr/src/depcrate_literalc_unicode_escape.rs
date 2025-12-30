// Generated macro for c_unicode_escape (function)
macro_rules! Depcrate_literalc_unicode_escape {
() => {
// Module: crate::literal
// Provides: {"c_unicode_escape"}
// Dependencies: {}
fn c_unicode_escape (n : Vec < u8 >) -> Option < CChar > { str :: from_utf8 (& n) . ok () . and_then (| i | u32 :: from_str_radix (i , 16) . ok ()) . and_then (char :: from_u32) . map (CChar :: Char) }
};
}
