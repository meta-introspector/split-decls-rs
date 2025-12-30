// Generated macro for MSR_LASTBRANCH_0_TO_IP (const)
macro_rules! Depcrate_msrMSR_LASTBRANCH_0_TO_IP {
() => {
// Module: crate::msr
// Provides: {"MSR_LASTBRANCH_0_TO_IP"}
// Dependencies: {}
# [doc = " Last Branch Record 0 (R/W)  One of 16 pairs of last branch record registers on  the last branch record stack (6C0H-6CFH). This  part of the stack contains pointers to the  destination instruction for one of the last 16  branches, exceptions, or interrupts that the  processor took. See Section 17.9, Last Branch, Interrupt, and  Exception Recording (Processors based on Intel  NetBurst® Microarchitecture)."] pub const MSR_LASTBRANCH_0_TO_IP : u32 = 0x6c0 ;
};
}
