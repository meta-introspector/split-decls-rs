// Generated macro for impl_endian (macro)
macro_rules! Depcrate_endianimpl_endian {
() => {
// Module: crate::endian
// Provides: {"impl_endian"}
// Dependencies: {}
macro_rules ! impl_endian { ($ endian : ident , $ base : ident , $ to_endian : ident , $ from_endian : ident , $ size : expr) => { impl Encoding <$ base > for $ endian <$ base > { const ZERO : Self = Self (0) ; } impl From <$ base > for $ endian <$ base > { # [inline] fn from (value : $ base) -> Self { Self ($ base ::$ to_endian (value)) } } impl From <$ endian <$ base >> for $ base { # [inline] fn from ($ endian (value) : $ endian <$ base >) -> Self { $ base ::$ from_endian (value) } } impl < const N : usize > FromArray < N , $ base > for $ endian <$ base > { fn from_array (value : & [$ base ; N]) -> [Self ; N] { let mut result : [$ endian <$ base >; N] = [$ endian :: ZERO ; N] ; for i in 0 .. N { result [i] = $ endian :: from (value [i]) ; } return result ; } } impl_array_encoding ! ($ endian , $ base , 1) ; impl_array_encoding ! ($ endian , $ base , 2) ; impl_array_encoding ! ($ endian , $ base , 3) ; impl_array_encoding ! ($ endian , $ base , 4) ; impl_array_encoding ! ($ endian , $ base , 8) ; } ; }
};
}
