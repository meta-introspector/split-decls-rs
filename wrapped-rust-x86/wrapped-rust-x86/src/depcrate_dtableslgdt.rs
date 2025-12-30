// Generated macro for lgdt (function)
macro_rules! Depcrate_dtableslgdt {
() => {
// Module: crate::dtables
// Provides: {"lgdt"}
// Dependencies: {}
# [doc = " Load the GDTR register with the specified base and limit."] # [doc = ""] # [doc = " # Safety"] # [doc = " Needs CPL 0."] pub unsafe fn lgdt < T > (gdt : & DescriptorTablePointer < T >) { asm ! ("lgdt ({0})" , in (reg) gdt , options (att_syntax)) ; }
};
}
