macro_rules! set_global_constant {
    () => {
        pub (crate) fn set_global_constant (llglobal : & Value , is_constant : bool) { LLVMSetGlobalConstant (llglobal , is_constant . to_llvm_bool ()) ; }
    };
}

set_global_constant!()