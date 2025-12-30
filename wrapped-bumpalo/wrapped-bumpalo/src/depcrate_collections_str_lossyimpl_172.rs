// Generated macro for impl_172 (impl)
macro_rules! Depcrate_collections_str_lossyimpl_172 {
() => {
// Module: crate::collections::str::lossy
// Provides: {"impl_172"}
// Dependencies: {}
impl < 'a > fmt :: Display for Utf8Lossy < 'a > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { if self . bytes . is_empty () { return "" . fmt (f) ; } for Utf8LossyChunk { valid , broken } in self . chunks () { if valid . len () == self . bytes . len () { assert ! (broken . is_empty ()) ; return valid . fmt (f) ; } f . write_str (valid) ? ; if ! broken . is_empty () { f . write_char (char :: REPLACEMENT_CHARACTER) ? ; } } Ok (()) } }
};
}
