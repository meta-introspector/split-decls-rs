// Generated macro for mul (function)
macro_rules! Depcrate_bellerophonmul {
() => {
// Module: crate::bellerophon
// Provides: {"mul"}
// Dependencies: {}
# [doc = " Multiply two normalized extended-precision floats, as if by `a*b`."] # [doc = ""] # [doc = " The precision is maximal when the numbers are normalized, however,"] # [doc = " decent precision will occur as long as both values have high bits"] # [doc = " set. The result is not normalized."] # [doc = ""] # [doc = " Algorithm:"] # [doc = "     1. Non-signed multiplication of mantissas (requires 2x as many bits as input)."] # [doc = "     2. Normalization of the result (not done here)."] # [doc = "     3. Addition of exponents."] pub fn mul (x : & ExtendedFloat , y : & ExtendedFloat) -> ExtendedFloat { debug_assert ! (x . mant >> 32 != 0) ; debug_assert ! (y . mant >> 32 != 0) ; const LOMASK : u64 = 0xffff_ffff ; let x1 = x . mant >> 32 ; let x0 = x . mant & LOMASK ; let y1 = y . mant >> 32 ; let y0 = y . mant & LOMASK ; let x1_y0 = x1 * y0 ; let x0_y1 = x0 * y1 ; let x0_y0 = x0 * y0 ; let x1_y1 = x1 * y1 ; let mut tmp = (x1_y0 & LOMASK) + (x0_y1 & LOMASK) + (x0_y0 >> 32) ; tmp += 1 << (32 - 1) ; ExtendedFloat { mant : x1_y1 + (x1_y0 >> 32) + (x0_y1 >> 32) + (tmp >> 32) , exp : x . exp + y . exp + 64 , } }
};
}
