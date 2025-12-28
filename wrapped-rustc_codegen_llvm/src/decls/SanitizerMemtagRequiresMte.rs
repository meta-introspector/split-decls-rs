macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! SanitizerMemtagRequiresMte {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_llvm_sanitizer_memtag_requires_mte)] pub (crate) struct SanitizerMemtagRequiresMte ;
    };
}

SanitizerMemtagRequiresMte!()