// Generated macro for MSR_LER_TO_LIP (const)
macro_rules! Depcrate_msrMSR_LER_TO_LIP {
() => {
// Module: crate::msr
// Provides: {"MSR_LER_TO_LIP"}
// Dependencies: {}
# [doc = " Last Exception Record To Linear IP (R)  This area contains a pointer to the target of the last branch instruction  that the processor executed prior to the last exception that was  generated or the last interrupt that was handled. See Section 17.11, Last Branch, Interrupt, and Exception Recording  (Pentium M Processors)  and Section 17.12.2, Last Branch and Last  Exception MSRs."] pub const MSR_LER_TO_LIP : u32 = 0x1dd ;
};
}
