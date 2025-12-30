// Generated macro for parse_module (function)
macro_rules! Depcrate_back_ltoparse_module {
() => {
// Module: crate::back::lto
// Provides: {"parse_module"}
// Dependencies: {}
pub (crate) fn parse_module < 'a > (cx : & 'a llvm :: Context , name : & CStr , data : & [u8] , dcx : DiagCtxtHandle < '_ > ,) -> & 'a llvm :: Module { unsafe { llvm :: LLVMRustParseBitcodeForLTO (cx , data . as_ptr () , data . len () , name . as_ptr ()) . unwrap_or_else (| | write :: llvm_err (dcx , LlvmError :: ParseBitcode)) } }
};
}
