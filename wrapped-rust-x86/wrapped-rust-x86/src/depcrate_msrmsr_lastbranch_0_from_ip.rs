// Generated macro for MSR_LASTBRANCH_0_FROM_IP (const)
macro_rules! Depcrate_msrMSR_LASTBRANCH_0_FROM_IP {
() => {
// Module: crate::msr
// Provides: {"MSR_LASTBRANCH_0_FROM_IP"}
// Dependencies: {}
# [doc = " Last Branch Record 0 From IP (R/W) One of eight pairs of last branch record registers on the last branch  record stack. This part of the stack contains pointers to the source  instruction for one of the last eight branches, exceptions, or  interrupts taken by the processor. See also: Last Branch Record Stack TOS at 1C9H Section 17.11, Last Branch, Interrupt, and Exception Recording  (Pentium M Processors)."] pub const MSR_LASTBRANCH_0_FROM_IP : u32 = 0x40 ;
};
}
