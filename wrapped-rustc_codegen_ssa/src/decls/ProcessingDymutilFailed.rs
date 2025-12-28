macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! ProcessingDymutilFailed {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_processing_dymutil_failed)] # [note] pub (crate) struct ProcessingDymutilFailed { pub status : ExitStatus , pub output : String , }
    };
}

ProcessingDymutilFailed!()