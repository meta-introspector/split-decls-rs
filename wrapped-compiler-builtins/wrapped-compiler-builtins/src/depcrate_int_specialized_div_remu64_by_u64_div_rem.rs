// Generated macro for u64_by_u64_div_rem (function)
macro_rules! Depcrate_int_specialized_div_remu64_by_u64_div_rem {
() => {
// Module: crate::int::specialized_div_rem
// Provides: {"u64_by_u64_div_rem"}
// Dependencies: {}
# [doc = " Divides `duo` by `div` and returns a tuple of the quotient and the remainder."] # [doc = " `checked_div` and `checked_rem` are used to avoid bringing in panic function"] # [doc = " dependencies."] # [inline] fn u64_by_u64_div_rem (duo : u64 , div : u64) -> (u64 , u64) { if let Some (quo) = duo . checked_div (div) && let Some (rem) = duo . checked_rem (div) { return (quo , rem) ; } zero_div_fn () }
};
}
