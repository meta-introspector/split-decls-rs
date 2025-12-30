// Generated macro for rdmsr (function)
macro_rules! Depcrate_msrrdmsr {
() => {
// Module: crate::msr
// Provides: {"rdmsr"}
// Dependencies: {}
# [doc = " Read 64 bits msr register."] # [doc = ""] # [doc = " # Safety"] # [doc = " Needs CPL 0."] # [allow (unused_mut)] pub unsafe fn rdmsr (msr : u32) -> u64 { let (high , low) : (u32 , u32) ; asm ! ("rdmsr" , out ("eax") low , out ("edx") high , in ("ecx") msr) ; ((high as u64) << 32) | (low as u64) }
};
}
