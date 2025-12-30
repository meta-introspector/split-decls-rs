// Generated macro for MSR_EBC_FREQUENCY_ID (const)
macro_rules! Depcrate_msrMSR_EBC_FREQUENCY_ID {
() => {
// Module: crate::msr
// Provides: {"MSR_EBC_FREQUENCY_ID"}
// Dependencies: {}
# [doc = " Processor Frequency Configuration The bit field layout of this MSR varies according to  the MODEL value in the CPUID version  information. The following bit field layout applies to Pentium 4 and Xeon Processors with MODEL  encoding equal or greater than 2.  (R) The field Indicates the current processor  frequency configuration."] pub const MSR_EBC_FREQUENCY_ID : u32 = 0x2c ;
};
}
