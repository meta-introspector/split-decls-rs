macro_rules! Finalize {
    () => {
        # [derive (Diagnostic)] # [diag (incremental_finalize)] pub (crate) struct Finalize < 'a > { pub path : & 'a Path , pub err : std :: io :: Error , }
    };
}

Finalize!();