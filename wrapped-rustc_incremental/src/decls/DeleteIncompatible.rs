macro_rules! DeleteIncompatible {
    () => {
        # [derive (Diagnostic)] # [diag (incremental_delete_incompatible)] pub (crate) struct DeleteIncompatible { pub path : PathBuf , pub err : std :: io :: Error , }
    };
}

DeleteIncompatible!()