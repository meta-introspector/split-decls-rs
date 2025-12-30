// Generated macro for impl_16 (impl)
macro_rules! Depcrateimpl_16 {
() => {
// Module: crate
// Provides: {"impl_16"}
// Dependencies: {}
impl < I > FromRadix10Checked for I where I : Zero + One + FromRadix10 + CheckedMul + CheckedAdd + MaxNumDigits , { fn from_radix_10_checked (text : & [u8]) -> (Option < I > , usize) { let max_safe_digits = max (1 , I :: max_num_digits_negative (nth (10))) - 1 ; let (number , mut index) = I :: from_radix_10 (& text [.. min (text . len () , max_safe_digits)]) ; let mut number = Some (number) ; while index != text . len () { if let Some (digit) = ascii_to_digit (text [index]) { number = number . and_then (| n | n . checked_mul (& nth (10))) ; number = number . and_then (| n | n . checked_add (& digit)) ; index += 1 ; } else { break ; } } (number , index) } }
};
}
