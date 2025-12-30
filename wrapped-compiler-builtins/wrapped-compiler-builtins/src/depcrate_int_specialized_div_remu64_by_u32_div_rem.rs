// Generated macro for u64_by_u32_div_rem (function)
macro_rules! Depcrate_int_specialized_div_remu64_by_u32_div_rem {
() => {
// Module: crate::int::specialized_div_rem
// Provides: {"u64_by_u32_div_rem"}
// Dependencies: {}
# [doc = " Divides `duo` by `div` and returns a tuple of the quotient and the remainder."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " If the quotient does not fit in a `u32`, a floating point exception occurs."] # [doc = " If `div == 0`, then a division by zero exception occurs."] # [cfg (all (not (feature = "no-asm") , target_arch = "x86"))] # [inline] unsafe fn u64_by_u32_div_rem (duo : u64 , div : u32) -> (u32 , u32) { let duo_lo = duo as u32 ; let duo_hi = (duo >> 32) as u32 ; let quo : u32 ; let rem : u32 ; unsafe { core :: arch :: asm ! ("div {0}" , in (reg) div , inlateout ("rax") duo_lo => quo , inlateout ("rdx") duo_hi => rem , options (att_syntax , pure , nomem , nostack)) ; } (quo , rem) }
};
}
