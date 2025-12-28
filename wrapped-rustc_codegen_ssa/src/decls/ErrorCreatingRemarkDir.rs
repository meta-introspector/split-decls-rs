macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! ErrorCreatingRemarkDir {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_error_creating_remark_dir)] pub (crate) struct ErrorCreatingRemarkDir { pub error : std :: io :: Error , }
    };
}

ErrorCreatingRemarkDir!()