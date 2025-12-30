// Generated macro for pow (function)
macro_rules! Depcrate_powpow {
() => {
// Module: crate::pow
// Provides: {"pow"}
// Dependencies: {}
# [doc = " Raises a value to the power of exp, using exponentiation by squaring."] # [doc = ""] # [doc = " Note that `0⁰` (`pow(0, 0)`) returns `1`. Mathematically this is undefined."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " use num_traits::pow;"] # [doc = ""] # [doc = " assert_eq!(pow(2i8, 4), 16);"] # [doc = " assert_eq!(pow(6u8, 3), 216);"] # [doc = " assert_eq!(pow(0u8, 0), 1); // Be aware if this case affects you"] # [doc = " ```"] # [inline] pub fn pow < T : Clone + One + Mul < T , Output = T > > (mut base : T , mut exp : usize) -> T { if exp == 0 { return T :: one () ; } while exp & 1 == 0 { base = base . clone () * base ; exp >>= 1 ; } if exp == 1 { return base ; } let mut acc = base . clone () ; while exp > 1 { exp >>= 1 ; base = base . clone () * base ; if exp & 1 == 1 { acc = acc * base . clone () ; } } acc }
};
}
