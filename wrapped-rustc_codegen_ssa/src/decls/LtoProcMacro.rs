macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! LtoProcMacro {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_lto_proc_macro)] pub (crate) struct LtoProcMacro ;
    };
}

LtoProcMacro!();