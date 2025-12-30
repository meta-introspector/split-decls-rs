// Generated macro for other_942 (other)
macro_rules! Depcrate_llvm_enzyme_ffiother_942 {
() => {
// Module: crate::llvm::enzyme_ffi
// Provides: {"other_942"}
// Dependencies: {}
unsafe extern "C" { pub (crate) fn LLVMDumpModule (M : & Module) ; pub (crate) fn LLVMDumpValue (V : & Value) ; pub (crate) fn LLVMGetFunctionCallConv (F : & Value) -> c_uint ; pub (crate) fn LLVMGetReturnType (T : & Type) -> & Type ; pub (crate) fn LLVMGetParams (Fnc : & Value , params : * mut & Value) ; pub (crate) fn LLVMGetNamedFunction (M : & Module , Name : * const c_char) -> Option < & Value > ; }
};
}
