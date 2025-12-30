// Generated macro for macro_220 (macro)
macro_rules! Depcrate_int_udivmacro_220 {
() => {
// Module: crate::int::udiv
// Provides: {"macro_220"}
// Dependencies: {}
# [cfg (target_arch = "avr")] intrinsics ! { # [doc = " Returns `n / d` and `n % d` packed together."] # [doc = ""] # [doc = " Ideally we'd use `-> (u32, u32)` or some kind of a packed struct, but"] # [doc = " both force a stack allocation, while our result has to be in R18:R26."] pub extern "C" fn __udivmodsi4 (n : u32 , d : u32) -> u64 { let (div , rem) = u32_div_rem (n , d) ; ((rem as u64) << 32) | (div as u64) } # [unsafe (naked)] pub unsafe extern "custom" fn __udivmodqi4 () { core :: arch :: naked_asm ! ("clr R25" , "ldi R23, 8" , "1:" , "lsl R24" , "rol R25" , "cp  R25, R22" , "brlo 2f" , "sub R25, R22" , "sbr R24, 1" , "2:" , "dec R23" , "brne 1b" , "ret" ,) ; } # [unsafe (naked)] pub unsafe extern "C" fn __udivmodhi4 () { core :: arch :: naked_asm ! ("mov R26, R22" , "mov R27, R23" , "mov R22, R24" , "mov R23, R25" , "clr R24" , "clr R25" , "ldi R21, 16" , "1:" , "lsl R22" , "rol R23" , "rol R24" , "rol R25" , "cp  R24, R26" , "cpc R25, R27" , "brlo 2f" , "sub R24, R26" , "sbc R25, R27" , "sbr R22, 1" , "2:" , "dec R21" , "brne 1b" , "ret" ,) ; } }
};
}
