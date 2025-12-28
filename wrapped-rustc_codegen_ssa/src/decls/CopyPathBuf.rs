macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! CopyPathBuf {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_copy_path_buf)] pub (crate) struct CopyPathBuf { pub source_file : PathBuf , pub output_path : PathBuf , pub error : Error , }
    };
}

CopyPathBuf!()