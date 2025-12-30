// Generated macro for hex (macro)
macro_rules! Depcratehex {
() => {
// Module: crate
// Provides: {"hex"}
// Dependencies: {}
# [doc = " Converts a sequence of hexadecimal string literals to a byte array at compile time."] # [doc = ""] # [doc = " See the crate-level docs for more information."] # [macro_export] macro_rules ! hex { ($ ($ s : literal) *) => { { const STRINGS : & [&'static [u8]] = & [$ ($ s . as_bytes () ,) *] ; const { $ crate :: decode ::< { $ crate :: len (STRINGS) } > (STRINGS) . expect ("Output array length should be correct") } } } ; }
};
}
