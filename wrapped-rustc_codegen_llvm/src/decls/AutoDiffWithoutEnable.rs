macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! AutoDiffWithoutEnable {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_llvm_autodiff_without_enable)] pub (crate) struct AutoDiffWithoutEnable ;
    };
}

AutoDiffWithoutEnable!();