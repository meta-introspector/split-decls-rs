macro_rules! DeleteFull {
    () => {
        # [derive (Diagnostic)] # [diag (incremental_delete_full)] pub (crate) struct DeleteFull < 'a > { pub path : & 'a Path , pub err : std :: io :: Error , }
    };
}

DeleteFull!();