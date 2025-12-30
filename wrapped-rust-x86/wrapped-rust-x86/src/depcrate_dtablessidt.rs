// Generated macro for sidt (function)
macro_rules! Depcrate_dtablessidt {
() => {
// Module: crate::dtables
// Provides: {"sidt"}
// Dependencies: {}
# [doc = " Retrieve base and limit from the IDTR register."] # [doc = ""] # [doc = " # Safety"] # [doc = " Needs CPL 0."] pub unsafe fn sidt < T > (idt : & mut DescriptorTablePointer < T >) { asm ! ("sidt ({0})" , in (reg) idt as * mut DescriptorTablePointer < T >, options (att_syntax)) ; }
};
}
