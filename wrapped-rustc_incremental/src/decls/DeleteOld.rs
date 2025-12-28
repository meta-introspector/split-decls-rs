macro_rules! DeleteOld {
    () => {
        # [derive (Diagnostic)] # [diag (incremental_delete_old)] pub (crate) struct DeleteOld < 'a > { pub name : & 'a str , pub path : PathBuf , pub err : std :: io :: Error , }
    };
}

DeleteOld!()