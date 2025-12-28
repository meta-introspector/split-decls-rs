macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! SanitizerKcfiArityRequiresLLVM2100 {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_llvm_sanitizer_kcfi_arity_requires_llvm_21_0_0)] pub (crate) struct SanitizerKcfiArityRequiresLLVM2100 ;
    };
}

SanitizerKcfiArityRequiresLLVM2100!()