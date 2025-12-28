macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! UnableToRun {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_unable_to_run)] pub (crate) struct UnableToRun < 'a > { pub util : & 'a str , pub error : Error , }
    };
}

UnableToRun!();