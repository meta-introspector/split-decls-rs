macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! FromLlvmDiag {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_llvm_from_llvm_diag)] pub (crate) struct FromLlvmDiag { pub message : String , }
    };
}

FromLlvmDiag!()