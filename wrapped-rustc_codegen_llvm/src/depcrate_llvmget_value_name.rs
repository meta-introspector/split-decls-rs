// Generated macro for get_value_name (function)
macro_rules! Depcrate_llvmget_value_name {
() => {
// Module: crate::llvm
// Provides: {"get_value_name"}
// Dependencies: {}
# [doc = " Safe wrapper for `LLVMGetValueName2`"] # [doc = " Needs to allocate the value, because `set_value_name` will invalidate"] # [doc = " the pointer."] pub (crate) fn get_value_name (value : & Value) -> Vec < u8 > { unsafe { let mut len = 0 ; let data = LLVMGetValueName2 (value , & mut len) ; std :: slice :: from_raw_parts (data . cast () , len) . to_vec () } }
};
}
