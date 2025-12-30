// Generated macro for stac (function)
macro_rules! Depcrate_bits32_eflagsstac {
() => {
// Module: crate::bits32::eflags
// Provides: {"stac"}
// Dependencies: {}
# [doc = " Sets the AC flag bit in EFLAGS register."] # [doc = ""] # [doc = " This may enable alignment checking of user-mode data accesses."] # [doc = " This allows explicit supervisor-mode data accesses to user-mode"] # [doc = " pages even if the SMAP bit is set in the CR4 register."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This instruction is only valid in Ring 0 and requires"] # [doc = " that the CPU supports the instruction (check CPUID)."] # [inline (always)] pub unsafe fn stac () { asm ! ("stac") ; }
};
}
