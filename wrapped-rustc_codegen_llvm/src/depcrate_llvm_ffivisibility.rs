// Generated macro for Visibility (enum)
macro_rules! Depcrate_llvm_ffiVisibility {
() => {
// Module: crate::llvm::ffi
// Provides: {"Visibility"}
// Dependencies: {}
# [doc = " Must match the layout of `LLVMVisibility`."] # [repr (C)] # [derive (Copy , Clone , PartialEq , TryFromU32)] pub (crate) enum Visibility { Default = 0 , Hidden = 1 , Protected = 2 , }
};
}
