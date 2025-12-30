// Generated macro for jis0208_range_encode (function)
macro_rules! Depcrate_datajis0208_range_encode {
() => {
// Module: crate::data
// Provides: {"jis0208_range_encode"}
// Dependencies: {}
# [inline (always)] pub fn jis0208_range_encode (bmp : u16) -> Option < usize > { let mut i = 0 ; while i < JIS0208_RANGE_TRIPLES . len () { let start = JIS0208_RANGE_TRIPLES [i + 2] as usize ; let length = JIS0208_RANGE_TRIPLES [i + 1] as usize ; let bmp_minus_start = (bmp as usize) . wrapping_sub (start) ; if bmp_minus_start < length { let offset = JIS0208_RANGE_TRIPLES [i] as usize ; return Some (bmp_minus_start + offset) ; } i += 3 ; } None }
};
}
