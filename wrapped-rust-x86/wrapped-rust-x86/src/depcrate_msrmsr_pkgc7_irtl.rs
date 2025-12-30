// Generated macro for MSR_PKGC7_IRTL (const)
macro_rules! Depcrate_msrMSR_PKGC7_IRTL {
() => {
// Module: crate::msr
// Provides: {"MSR_PKGC7_IRTL"}
// Dependencies: {}
# [doc = " Package C7 Interrupt Response Limit (R/W)  This MSR defines the budget allocated for the package to exit from  C7 to a C0 state, where interrupt request can be delivered to the  core and serviced. Additional core-exit latency amy be applicable  depending on the actual C-state the core is in.  Note: C-state values are processor specific C-state code names,  unrelated to MWAIT extension C-state parameters or ACPI C-States."] pub const MSR_PKGC7_IRTL : u32 = 0x60c ;
};
}
