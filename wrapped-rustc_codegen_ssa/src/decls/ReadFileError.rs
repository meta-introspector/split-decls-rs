macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! ReadFileError {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_read_file)] pub (crate) struct ReadFileError { pub message : std :: io :: Error , }
    };
}

ReadFileError!();