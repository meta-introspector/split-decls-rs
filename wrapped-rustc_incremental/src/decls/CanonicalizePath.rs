macro_rules! CanonicalizePath {
    () => {
        # [derive (Diagnostic)] # [diag (incremental_canonicalize_path)] pub (crate) struct CanonicalizePath { pub path : PathBuf , pub err : std :: io :: Error , }
    };
}

CanonicalizePath!();