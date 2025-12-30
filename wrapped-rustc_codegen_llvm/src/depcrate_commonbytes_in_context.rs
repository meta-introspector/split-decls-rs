// Generated macro for bytes_in_context (function)
macro_rules! Depcrate_commonbytes_in_context {
() => {
// Module: crate::common
// Provides: {"bytes_in_context"}
// Dependencies: {}
pub (crate) fn bytes_in_context < 'll > (llcx : & 'll llvm :: Context , bytes : & [u8]) -> & 'll Value { unsafe { let ptr = bytes . as_ptr () as * const c_char ; llvm :: LLVMConstStringInContext2 (llcx , ptr , bytes . len () , TRUE) } }
};
}
