macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! LinkScriptWriteFailure {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_link_script_write_failure)] pub (crate) struct LinkScriptWriteFailure { pub path : PathBuf , pub error : Error , }
    };
}

LinkScriptWriteFailure!()