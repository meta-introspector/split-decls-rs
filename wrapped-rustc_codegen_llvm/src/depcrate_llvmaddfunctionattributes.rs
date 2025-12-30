// Generated macro for AddFunctionAttributes (function)
macro_rules! Depcrate_llvmAddFunctionAttributes {
() => {
// Module: crate::llvm
// Provides: {"AddFunctionAttributes"}
// Dependencies: {}
pub (crate) fn AddFunctionAttributes < 'll > (llfn : & 'll Value , idx : AttributePlace , attrs : & [& 'll Attribute] ,) { unsafe { LLVMRustAddFunctionAttributes (llfn , idx . as_uint () , attrs . as_ptr () , attrs . len ()) ; } }
};
}
