macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! VersionScriptWriteFailure {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_version_script_write_failure)] pub (crate) struct VersionScriptWriteFailure { pub error : Error , }
    };
}

VersionScriptWriteFailure!();