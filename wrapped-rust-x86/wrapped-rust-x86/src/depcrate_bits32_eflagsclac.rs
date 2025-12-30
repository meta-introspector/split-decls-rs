// Generated macro for clac (function)
macro_rules! Depcrate_bits32_eflagsclac {
() => {
// Module: crate::bits32::eflags
// Provides: {"clac"}
// Dependencies: {}
# [doc = " Clears the AC flag bit in EFLAGS register."] # [doc = ""] # [doc = " This disables any alignment checking of user-mode data accesses."] # [doc = " If the SMAP bit is set in the CR4 register, this disallows"] # [doc = " explicit supervisor-mode data accesses to user-mode pages."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This instruction is only valid in Ring 0 and requires"] # [doc = " that the CPU supports the instruction (check CPUID)."] # [inline (always)] pub unsafe fn clac () { asm ! ("clac") ; }
};
}
