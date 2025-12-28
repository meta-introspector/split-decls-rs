macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! MismatchedDataLayout {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_llvm_mismatch_data_layout)] pub (crate) struct MismatchedDataLayout < 'a > { pub rustc_target : & 'a str , pub rustc_layout : & 'a str , pub llvm_target : & 'a str , pub llvm_layout : & 'a str , }
    };
}

MismatchedDataLayout!();