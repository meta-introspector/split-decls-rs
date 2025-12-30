// Generated macro for VmFail (enum)
macro_rules! Depcrate_vmxVmFail {
() => {
// Module: crate::vmx
// Provides: {"VmFail"}
// Dependencies: {}
# [doc = " Possible outcomes of VMfail pseudo-function used to convey VMX operation errors."] # [doc = ""] # [doc = " Definitions of all these pseudo-functions can be found in Intel SDM, Volume 3C, Section 30.2."] # [derive (Debug)] pub enum VmFail { # [doc = " VMCS pointer is valid, but some other error was encountered. Read"] # [doc = " VM-instruction error field of VMCS for more details."] VmFailValid , # [doc = " VMCS pointer is not valid."] VmFailInvalid , }
};
}
