// Generated macro for Result (type)
macro_rules! Depcrate_vmxResult {
() => {
// Module: crate::vmx
// Provides: {"Result"}
// Dependencies: {}
# [doc = " A specialized [`Result`](core::result::Result) type for VMX operations."] # [doc = ""] # [doc = " This type closely replicates VMX instruction conventions described in"] # [doc = " Intel SDM, Volume 3C, Section 30.2."] pub type Result < T > = core :: result :: Result < T , VmFail > ;
};
}
