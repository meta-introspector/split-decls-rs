// Generated macro for lidt (function)
macro_rules! Depcrate_dtableslidt {
() => {
// Module: crate::dtables
// Provides: {"lidt"}
// Dependencies: {}
# [doc = " Load the IDTR register with the specified base and limit."] # [doc = ""] # [doc = " # Safety"] # [doc = " Needs CPL 0."] pub unsafe fn lidt < T > (idt : & DescriptorTablePointer < T >) { asm ! ("lidt ({0})" , in (reg) idt , options (att_syntax)) ; }
};
}
