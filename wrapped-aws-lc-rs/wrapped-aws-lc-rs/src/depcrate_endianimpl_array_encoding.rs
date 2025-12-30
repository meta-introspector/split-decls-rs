// Generated macro for impl_array_encoding (macro)
macro_rules! Depcrate_endianimpl_array_encoding {
() => {
// Module: crate::endian
// Provides: {"impl_array_encoding"}
// Dependencies: {}
macro_rules ! impl_array_encoding { ($ endian : ident , $ base : ident , $ elems : expr) => { impl ArrayEncoding < [u8 ; $ elems * core :: mem :: size_of ::<$ base > ()] > for [$ endian <$ base >; $ elems] { fn as_byte_array (& self) -> & [u8 ; $ elems * core :: mem :: size_of ::<$ base > ()] { as_byte_slice (self) . try_into () . unwrap () } } } ; }
};
}
