// Generated macro for addhilo (function)
macro_rules! Depcrate_primitivesaddhilo {
() => {
// Module: crate::primitives
// Provides: {"addhilo"}
// Dependencies: {}
# [doc = " Adds wide numbers represented by pairs of (least significant word, most significant word)"] # [doc = " and returns the result in the same format `(lo, hi)`."] # [inline (always)] pub (crate) const fn addhilo (x_lo : Word , x_hi : Word , y_lo : Word , y_hi : Word) -> (Word , Word) { let res = (((x_hi as WideWord) << Word :: BITS) | (x_lo as WideWord)) + (((y_hi as WideWord) << Word :: BITS) | (y_lo as WideWord)) ; (res as Word , (res >> Word :: BITS) as Word) }
};
}
