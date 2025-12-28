macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! InsufficientVSCodeProduct {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_insufficient_vs_code_product)] pub (crate) struct InsufficientVSCodeProduct ;
    };
}

InsufficientVSCodeProduct!();