// Generated macro for ToLlvmBool (trait)
macro_rules! Depcrate_llvm_ffiToLlvmBool {
() => {
// Module: crate::llvm::ffi
// Provides: {"ToLlvmBool"}
// Dependencies: {}
# [doc = " Convenience trait to convert `bool` to `llvm::Bool` with an explicit method call."] # [doc = ""] # [doc = " Being able to write `b.to_llvm_bool()` is less noisy than `llvm::Bool::from(b)`,"] # [doc = " while being more explicit and less mistake-prone than something like `b.into()`."] pub (crate) trait ToLlvmBool : Copy { fn to_llvm_bool (self) -> llvm :: Bool ; }
};
}
