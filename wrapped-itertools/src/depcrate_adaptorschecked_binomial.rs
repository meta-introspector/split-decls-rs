// Generated macro for checked_binomial (function)
macro_rules! Depcrate_adaptorschecked_binomial {
() => {
// Module: crate::adaptors
// Provides: {"checked_binomial"}
// Dependencies: {}
pub (crate) fn checked_binomial (mut n : usize , mut k : usize) -> Option < usize > { if n < k { return Some (0) ; } k = (n - k) . min (k) ; let mut c = 1 ; for i in 1 ..= k { c = (c / i) . checked_mul (n) ? . checked_add ((c % i) . checked_mul (n) ? / i) ? ; n -= 1 ; } Some (c) }
};
}
