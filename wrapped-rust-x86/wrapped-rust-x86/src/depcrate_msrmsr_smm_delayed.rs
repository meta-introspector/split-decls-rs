// Generated macro for MSR_SMM_DELAYED (const)
macro_rules! Depcrate_msrMSR_SMM_DELAYED {
() => {
// Module: crate::msr
// Provides: {"MSR_SMM_DELAYED"}
// Dependencies: {}
# [doc = " SMM Delayed (SMM-RO) Reports the interruptible state of all logical processors in the  package . Available only while in SMM and  MSR_SMM_MCA_CAP\\[LONG_FLOW_INDICATION\\] == 1."] pub const MSR_SMM_DELAYED : u32 = 0x4e2 ;
};
}
