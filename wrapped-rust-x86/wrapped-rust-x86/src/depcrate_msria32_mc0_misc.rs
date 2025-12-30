// Generated macro for IA32_MC0_MISC (const)
macro_rules! Depcrate_msrIA32_MC0_MISC {
() => {
// Module: crate::msr
// Provides: {"IA32_MC0_MISC"}
// Dependencies: {}
# [doc = " See Section 15.3.2.4,  IA32_MCi_MISC MSRs. The IA32_MC0_MISC MSR is either not  implemented or does not contain additional  information if the MISCV flag in the  IA32_MC0_STATUS register is clear. When not implemented in the processor, all reads  and writes to this MSR will cause a general- protection exception."] pub const IA32_MC0_MISC : u32 = 0x403 ;
};
}
