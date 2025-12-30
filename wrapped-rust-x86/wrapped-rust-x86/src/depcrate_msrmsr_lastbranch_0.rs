// Generated macro for MSR_LASTBRANCH_0 (const)
macro_rules! Depcrate_msrMSR_LASTBRANCH_0 {
() => {
// Module: crate::msr
// Provides: {"MSR_LASTBRANCH_0"}
// Dependencies: {}
# [doc = " Last Branch Record 0 (R/W)  One of four last branch record registers on the last  branch record stack. It contains pointers to the  source and destination instruction for one of the  last four branches, exceptions, or interrupts that  the processor took. MSR_LASTBRANCH_0 through  MSR_LASTBRANCH_3 at 1DBH-1DEH are  available only on family 0FH, models 0H-02H.  They have been replaced by the MSRs at 680H- 68FH and 6C0H-6CFH."] pub const MSR_LASTBRANCH_0 : u32 = 0x1db ;
};
}
