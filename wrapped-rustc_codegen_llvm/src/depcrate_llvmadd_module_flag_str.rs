// Generated macro for add_module_flag_str (function)
macro_rules! Depcrate_llvmadd_module_flag_str {
() => {
// Module: crate::llvm
// Provides: {"add_module_flag_str"}
// Dependencies: {}
pub (crate) fn add_module_flag_str (module : & Module , merge_behavior : ModuleFlagMergeBehavior , key : & str , value : & str ,) { unsafe { LLVMRustAddModuleFlagString (module , merge_behavior , key . as_c_char_ptr () , key . len () , value . as_c_char_ptr () , value . len () ,) ; } }
};
}
