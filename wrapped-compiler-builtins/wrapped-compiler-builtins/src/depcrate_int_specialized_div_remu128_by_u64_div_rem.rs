// Generated macro for u128_by_u64_div_rem (function)
macro_rules! Depcrate_int_specialized_div_remu128_by_u64_div_rem {
() => {
// Module: crate::int::specialized_div_rem
// Provides: {"u128_by_u64_div_rem"}
// Dependencies: {}
# [doc = " Divides `duo` by `div` and returns a tuple of the quotient and the remainder."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " If the quotient does not fit in a `u64`, a floating point exception occurs."] # [doc = " If `div == 0`, then a division by zero exception occurs."] # [cfg (all (not (feature = "no-asm") , target_arch = "x86_64"))] # [inline] unsafe fn u128_by_u64_div_rem (duo : u128 , div : u64) -> (u64 , u64) { let duo_lo = duo as u64 ; let duo_hi = (duo >> 64) as u64 ; let quo : u64 ; let rem : u64 ; unsafe { core :: arch :: asm ! ("div {0}" , in (reg) div , inlateout ("rax") duo_lo => quo , inlateout ("rdx") duo_hi => rem , options (att_syntax , pure , nomem , nostack)) ; } (quo , rem) }
};
}
