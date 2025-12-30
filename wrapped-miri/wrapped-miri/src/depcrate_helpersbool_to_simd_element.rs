// Generated macro for bool_to_simd_element (function)
macro_rules! Depcrate_helpersbool_to_simd_element {
() => {
// Module: crate::helpers
// Provides: {"bool_to_simd_element"}
// Dependencies: {}
pub (crate) fn bool_to_simd_element (b : bool , size : Size) -> Scalar { let val = if b { - 1 } else { 0 } ; Scalar :: from_int (val , size) }
};
}
