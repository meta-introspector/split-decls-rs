// Generated macro for IA32_MC3_ADDR (const)
macro_rules! Depcrate_msrIA32_MC3_ADDR {
() => {
// Module: crate::msr
// Provides: {"IA32_MC3_ADDR"}
// Dependencies: {}
# [doc = " See Section 15.3.2.3, IA32_MCi_ADDR MSRs. The IA32_MC3_ADDR register is either not  implemented or contains no address if the ADDRV  flag in the IA32_MC3_STATUS register is clear. When not implemented in the processor, all reads  and writes to this MSR will cause a general- protection exception."] pub const IA32_MC3_ADDR : u32 = 0x40e ;
};
}
