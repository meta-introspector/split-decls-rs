macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! LinkExeUnexpectedError {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_link_exe_unexpected_error)] pub (crate) struct LinkExeUnexpectedError ;
    };
}

LinkExeUnexpectedError!()