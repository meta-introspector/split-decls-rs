// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/num/int_sqrt.rs
// Error: expected square brackets
// Problematic line: line 12

//! "Paul Zimmermann. Karatsuba Square Root. \[Research Report\] RR-3805,
//! INRIA. 1999, pp.8. (inria-00072854)"

/// This array stores the [integer square roots](
/// https://en.wikipedia.org/wiki/Integer_square_root) and remainders of each
/// [`u8`](prim@u8) value. For example, `U8_ISQRT_WITH_REMAINDER[17]` will be
/// `(4, 1)` because the integer square root of 17 is 4 and because 17 is 1
