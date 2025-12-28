macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! RlibArchiveBuildFailure {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_rlib_archive_build_failure)] pub (crate) struct RlibArchiveBuildFailure { pub path : PathBuf , pub error : Error , }
    };
}

RlibArchiveBuildFailure!();