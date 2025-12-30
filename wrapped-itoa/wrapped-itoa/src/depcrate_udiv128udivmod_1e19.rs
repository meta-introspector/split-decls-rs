// Generated macro for udivmod_1e19 (function)
macro_rules! Depcrate_udiv128udivmod_1e19 {
() => {
// Module: crate::udiv128
// Provides: {"udivmod_1e19"}
// Dependencies: {}
# [doc = " Divide `n` by 1e19 and return quotient and remainder"] # [doc = ""] # [doc = " Integer division algorithm is based on the following paper:"] # [doc = ""] # [doc = "   T. Granlund and P. Montgomery, “Division by Invariant Integers Using Multiplication”"] # [doc = "   in Proc. of the SIGPLAN94 Conference on Programming Language Design and"] # [doc = "   Implementation, 1994, pp. 61–72"] # [doc = ""] # [inline] # [cfg_attr (feature = "no-panic" , no_panic)] pub fn udivmod_1e19 (n : u128) -> (u128 , u64) { let d = 10_000_000_000_000_000_000_u64 ; let quot = if n < 1 << 83 { ((n >> 19) as u64 / (d >> 19)) as u128 } else { u128_mulhi (n , 156927543384667019095894735580191660403) >> 62 } ; let rem = (n - quot * d as u128) as u64 ; debug_assert_eq ! (quot , n / d as u128) ; debug_assert_eq ! (rem as u128 , n % d as u128) ; (quot , rem) }
};
}
