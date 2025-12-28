macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! ErrorWritingDEFFile {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_error_writing_def_file)] pub (crate) struct ErrorWritingDEFFile { pub error : std :: io :: Error , }
    };
}

ErrorWritingDEFFile!();