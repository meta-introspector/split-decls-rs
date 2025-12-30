// Generated macro for wrmsr (function)
macro_rules! Depcrate_msrwrmsr {
() => {
// Module: crate::msr
// Provides: {"wrmsr"}
// Dependencies: {}
# [doc = " Write 64 bits to msr register."] # [doc = ""] # [doc = " # Safety"] # [doc = " Needs CPL 0."] pub unsafe fn wrmsr (msr : u32 , value : u64) { let low = value as u32 ; let high = (value >> 32) as u32 ; asm ! ("wrmsr" , in ("ecx") msr , in ("eax") low , in ("edx") high) ; }
};
}
