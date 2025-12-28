macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! CreateTempDir {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_create_temp_dir)] pub (crate) struct CreateTempDir { pub error : Error , }
    };
}

CreateTempDir!();