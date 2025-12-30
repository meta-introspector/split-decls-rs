// Generated macro for add_module_flag_u32 (function)
macro_rules! Depcrate_llvmadd_module_flag_u32 {
() => {
// Module: crate::llvm
// Provides: {"add_module_flag_u32"}
// Dependencies: {}
pub (crate) fn add_module_flag_u32 (module : & Module , merge_behavior : ModuleFlagMergeBehavior , key : & str , value : u32 ,) { unsafe { LLVMRustAddModuleFlagU32 (module , merge_behavior , key . as_c_char_ptr () , key . len () , value) ; } }
};
}
