// Generated macro for SetFunctionCallConv (function)
macro_rules! Depcrate_llvmSetFunctionCallConv {
() => {
// Module: crate::llvm
// Provides: {"SetFunctionCallConv"}
// Dependencies: {}
pub (crate) fn SetFunctionCallConv (fn_ : & Value , cc : CallConv) { unsafe { LLVMSetFunctionCallConv (fn_ , cc as c_uint) ; } }
};
}
