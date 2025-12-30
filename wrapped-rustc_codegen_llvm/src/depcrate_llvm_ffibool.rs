// Generated macro for Bool (struct)
macro_rules! Depcrate_llvm_ffiBool {
() => {
// Module: crate::llvm::ffi
// Provides: {"Bool"}
// Dependencies: {}
# [doc = " In the LLVM-C API, boolean values are passed as `typedef int LLVMBool`,"] # [doc = " which has a different ABI from Rust or C++ `bool`."] # [doc = ""] # [doc = " This wrapper does not implement `PartialEq`."] # [doc = " To test the underlying boolean value, use [`Self::is_true`]."] # [derive (Clone , Copy)] # [repr (transparent)] pub (crate) struct Bool { value : c_int , }
};
}
