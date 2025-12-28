macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! UnknownCompression {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_llvm_unknown_debuginfo_compression)] pub (crate) struct UnknownCompression { pub algorithm : & 'static str , }
    };
}

UnknownCompression!()