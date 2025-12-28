macro_rules! create_pgo_func_name_var {
    () => {
        pub (crate) fn create_pgo_func_name_var < 'll > (llfn : & 'll llvm :: Value , mangled_fn_name : & str ,) -> & 'll llvm :: Value { unsafe { llvm :: LLVMRustCoverageCreatePGOFuncNameVar (llfn , mangled_fn_name . as_c_char_ptr () , mangled_fn_name . len () ,) } }
    };
}

create_pgo_func_name_var!()