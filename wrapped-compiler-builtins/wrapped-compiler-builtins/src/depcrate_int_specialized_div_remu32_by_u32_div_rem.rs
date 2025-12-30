// Generated macro for u32_by_u32_div_rem (function)
macro_rules! Depcrate_int_specialized_div_remu32_by_u32_div_rem {
() => {
// Module: crate::int::specialized_div_rem
// Provides: {"u32_by_u32_div_rem"}
// Dependencies: {}
# [doc = " Divides `duo` by `div` and returns a tuple of the quotient and the remainder."] # [doc = " `checked_div` and `checked_rem` are used to avoid bringing in panic function"] # [doc = " dependencies."] # [inline] # [allow (dead_code)] fn u32_by_u32_div_rem (duo : u32 , div : u32) -> (u32 , u32) { if let Some (quo) = duo . checked_div (div) && let Some (rem) = duo . checked_rem (div) { return (quo , rem) ; } zero_div_fn () }
};
}
