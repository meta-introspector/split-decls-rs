macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! ArchiveBuildFailure {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_archive_build_failure)] pub struct ArchiveBuildFailure { pub path : PathBuf , pub error : std :: io :: Error , }
    };
}

ArchiveBuildFailure!()