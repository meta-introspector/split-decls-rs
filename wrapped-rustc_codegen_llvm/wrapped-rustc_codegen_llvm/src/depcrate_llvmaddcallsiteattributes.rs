// Generated macro for AddCallSiteAttributes (function)
macro_rules! Depcrate_llvmAddCallSiteAttributes {
() => {
// Module: crate::llvm
// Provides: {"AddCallSiteAttributes"}
// Dependencies: {}
pub (crate) fn AddCallSiteAttributes < 'll > (callsite : & 'll Value , idx : AttributePlace , attrs : & [& 'll Attribute] ,) { unsafe { LLVMRustAddCallSiteAttributes (callsite , idx . as_uint () , attrs . as_ptr () , attrs . len ()) ; } }
};
}
