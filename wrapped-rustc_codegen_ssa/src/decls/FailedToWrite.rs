macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! FailedToWrite {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_failed_to_write)] pub (crate) struct FailedToWrite { pub path : PathBuf , pub error : Error , }
    };
}

FailedToWrite!();