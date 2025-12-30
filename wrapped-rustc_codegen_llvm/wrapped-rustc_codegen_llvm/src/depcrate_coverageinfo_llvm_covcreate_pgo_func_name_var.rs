// Generated macro for create_pgo_func_name_var (function)
macro_rules! Depcrate_coverageinfo_llvm_covcreate_pgo_func_name_var {
() => {
// Module: crate::coverageinfo::llvm_cov
// Provides: {"create_pgo_func_name_var"}
// Dependencies: {}
pub (crate) fn create_pgo_func_name_var < 'll > (llfn : & 'll llvm :: Value , mangled_fn_name : & str ,) -> & 'll llvm :: Value { unsafe { llvm :: LLVMRustCoverageCreatePGOFuncNameVar (llfn , mangled_fn_name . as_c_char_ptr () , mangled_fn_name . len () ,) } }
};
}
