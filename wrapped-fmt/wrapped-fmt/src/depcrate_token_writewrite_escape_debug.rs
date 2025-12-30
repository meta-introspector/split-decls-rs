// Generated macro for write_escape_debug (function)
macro_rules! Depcrate_token_writewrite_escape_debug {
() => {
// Module: crate::token_write
// Provides: {"write_escape_debug"}
// Dependencies: {}
fn write_escape_debug (input : impl fmt :: Display , output : impl FnMut (& str) -> fmt :: Result ,) -> fmt :: Result { struct Writer < F > (F) ; impl < F : FnMut (& str) -> fmt :: Result > fmt :: Write for Writer < F > { fn write_str (& mut self , input : & str) -> fmt :: Result { let mut from = 0 ; for (i , c) in input . char_indices () { let esc = c . escape_debug () ; if esc . len () > 1 { let flush = & input [from .. i] ; if flush . len () > 0 { (self . 0) (flush) ? ; } let mut buf = [0 ; 4] ; for c in esc { (self . 0) (c . encode_utf8 (& mut buf)) ? ; } from = i + c . len_utf8 () ; } } let flush = & input [from ..] ; if flush . len () > 0 { (self . 0) (flush) } else { Ok (()) } } } write ! (Writer (output) , "{}" , input) }
};
}
