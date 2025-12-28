macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! LibDefWriteFailure {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_lib_def_write_failure)] pub (crate) struct LibDefWriteFailure { pub error : Error , }
    };
}

LibDefWriteFailure!()