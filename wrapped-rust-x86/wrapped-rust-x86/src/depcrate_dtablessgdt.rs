// Generated macro for sgdt (function)
macro_rules! Depcrate_dtablessgdt {
() => {
// Module: crate::dtables
// Provides: {"sgdt"}
// Dependencies: {}
# [doc = " Retrieve base and limit from the GDTR register."] # [doc = ""] # [doc = " # Safety"] # [doc = " Needs CPL 0."] pub unsafe fn sgdt < T > (idt : & mut DescriptorTablePointer < T >) { asm ! ("sgdt ({0})" , in (reg) idt as * mut DescriptorTablePointer < T >, options (att_syntax)) ; }
};
}
