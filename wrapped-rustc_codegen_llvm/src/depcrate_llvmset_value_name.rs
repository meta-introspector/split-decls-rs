// Generated macro for set_value_name (function)
macro_rules! Depcrate_llvmset_value_name {
() => {
// Module: crate::llvm
// Provides: {"set_value_name"}
// Dependencies: {}
# [doc = " Safe wrapper for `LLVMSetValueName2` from a byte slice"] pub (crate) fn set_value_name (value : & Value , name : & [u8]) { unsafe { let data = name . as_c_char_ptr () ; LLVMSetValueName2 (value , data , name . len ()) ; } }
};
}
