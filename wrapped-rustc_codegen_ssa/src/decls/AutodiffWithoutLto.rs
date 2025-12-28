macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! AutodiffWithoutLto {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_autodiff_without_lto)] pub struct AutodiffWithoutLto ;
    };
}

AutodiffWithoutLto!();