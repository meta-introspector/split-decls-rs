macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! LinkerNotFound {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_linker_not_found)] # [note] pub (crate) struct LinkerNotFound { pub linker_path : PathBuf , pub error : Error , }
    };
}

LinkerNotFound!()