// Generated macro for c_raw_escape (function)
macro_rules! Depcrate_literalc_raw_escape {
() => {
// Module: crate::literal
// Provides: {"c_raw_escape"}
// Dependencies: {}
fn c_raw_escape (n : Vec < u8 > , radix : u32) -> Option < CChar > { str :: from_utf8 (& n) . ok () . and_then (| i | u64 :: from_str_radix (i , radix) . ok ()) . map (| i | match i { 0 ..= 0x7f => CChar :: Char (i as u8 as char) , _ => CChar :: Raw (i) , }) }
};
}
