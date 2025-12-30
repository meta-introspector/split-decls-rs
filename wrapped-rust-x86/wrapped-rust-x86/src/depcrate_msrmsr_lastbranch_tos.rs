// Generated macro for MSR_LASTBRANCH_TOS (const)
macro_rules! Depcrate_msrMSR_LASTBRANCH_TOS {
() => {
// Module: crate::msr
// Provides: {"MSR_LASTBRANCH_TOS"}
// Dependencies: {}
# [doc = " Last Branch Record Stack TOS (R/W)  Contains an index (0-3 or 0-15) that points to the  top of the last branch record stack (that is, that points the index of the MSR containing the most  recent branch record). See Section 17.9.2, LBR Stack for Processors Based on Intel NetBurst® Microarchitecture ; and  addresses 1DBH-1DEH and 680H-68FH."] pub const MSR_LASTBRANCH_TOS : u32 = 0x1da ;
};
}
