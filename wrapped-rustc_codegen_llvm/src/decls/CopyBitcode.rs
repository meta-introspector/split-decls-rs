macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! CopyBitcode {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_llvm_copy_bitcode)] pub (crate) struct CopyBitcode { pub err : std :: io :: Error , }
    };
}

CopyBitcode!()