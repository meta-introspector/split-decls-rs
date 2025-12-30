// Generated macro for impl_173 (impl)
macro_rules! Depcrate_collections_str_lossyimpl_173 {
() => {
// Module: crate::collections::str::lossy
// Provides: {"impl_173"}
// Dependencies: {}
impl < 'a > fmt :: Debug for Utf8Lossy < 'a > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . write_char ('"') ? ; for Utf8LossyChunk { valid , broken } in self . chunks () { { let mut from = 0 ; for (i , c) in valid . char_indices () { let esc = c . escape_debug () ; if esc . len () != 1 { f . write_str (& valid [from .. i]) ? ; for c in esc { f . write_char (c) ? ; } from = i + c . len_utf8 () ; } } f . write_str (& valid [from ..]) ? ; } for & b in broken { write ! (f , "\\x{:02x}" , b) ? ; } } f . write_char ('"') } }
};
}
