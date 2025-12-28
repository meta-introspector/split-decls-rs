macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! UnableToRunDsymutil {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_unable_to_run_dsymutil)] pub (crate) struct UnableToRunDsymutil { pub error : Error , }
    };
}

UnableToRunDsymutil!();