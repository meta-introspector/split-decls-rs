// Generated macro for impl_13 (impl)
macro_rules! Depcrateimpl_13 {
() => {
// Module: crate
// Provides: {"impl_13"}
// Dependencies: {}
impl < I > FromRadix10 for I where I : Zero + One + AddAssign + MulAssign , { fn from_radix_10 (text : & [u8]) -> (Self , usize) { let mut index = 0 ; let mut number = I :: zero () ; while index != text . len () { if let Some (digit) = ascii_to_digit (text [index]) { number *= nth (10) ; number += digit ; index += 1 ; } else { break ; } } (number , index) } }
};
}
