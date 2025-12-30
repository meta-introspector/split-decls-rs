// Generated macro for get_param (function)
macro_rules! Depcrate_llvmget_param {
() => {
// Module: crate::llvm
// Provides: {"get_param"}
// Dependencies: {}
# [doc = " Safe wrapper around `LLVMGetParam`, because segfaults are no fun."] pub (crate) fn get_param (llfn : & Value , index : c_uint) -> & Value { unsafe { assert ! (index < LLVMCountParams (llfn) , "out of bounds argument access: {} out of {} arguments" , index , LLVMCountParams (llfn)) ; LLVMGetParam (llfn , index) } }
};
}
