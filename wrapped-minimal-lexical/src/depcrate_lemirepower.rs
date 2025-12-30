// Generated macro for power (function)
macro_rules! Depcrate_lemirepower {
() => {
// Module: crate::lemire
// Provides: {"power"}
// Dependencies: {}
# [doc = " Calculate a base 2 exponent from a decimal exponent."] # [doc = " This uses a pre-computed integer approximation for"] # [doc = " log2(10), where 217706 / 2^16 is accurate for the"] # [doc = " entire range of non-finite decimal exponents."] # [inline] fn power (q : i32) -> i32 { (q . wrapping_mul (152_170 + 65536) >> 16) + 63 }
};
}
