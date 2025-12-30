// Generated macro for checked_pow (function)
macro_rules! Depcrate_powchecked_pow {
() => {
// Module: crate::pow
// Provides: {"checked_pow"}
// Dependencies: {}
# [doc = " Raises a value to the power of exp, returning `None` if an overflow occurred."] # [doc = ""] # [doc = " Note that `0⁰` (`checked_pow(0, 0)`) returns `Some(1)`. Mathematically this is undefined."] # [doc = ""] # [doc = " Otherwise same as the `pow` function."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " use num_traits::checked_pow;"] # [doc = ""] # [doc = " assert_eq!(checked_pow(2i8, 4), Some(16));"] # [doc = " assert_eq!(checked_pow(7i8, 8), None);"] # [doc = " assert_eq!(checked_pow(7u32, 8), Some(5_764_801));"] # [doc = " assert_eq!(checked_pow(0u32, 0), Some(1)); // Be aware if this case affect you"] # [doc = " ```"] # [inline] pub fn checked_pow < T : Clone + One + CheckedMul > (mut base : T , mut exp : usize) -> Option < T > { if exp == 0 { return Some (T :: one ()) ; } while exp & 1 == 0 { base = base . checked_mul (& base) ? ; exp >>= 1 ; } if exp == 1 { return Some (base) ; } let mut acc = base . clone () ; while exp > 1 { exp >>= 1 ; base = base . checked_mul (& base) ? ; if exp & 1 == 1 { acc = acc . checked_mul (& base) ? ; } } Some (acc) }
};
}
