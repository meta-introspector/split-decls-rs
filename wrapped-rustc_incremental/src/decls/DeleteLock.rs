macro_rules! DeleteLock {
    () => {
        # [derive (Diagnostic)] # [diag (incremental_delete_lock)] pub (crate) struct DeleteLock < 'a > { pub path : & 'a Path , pub err : std :: io :: Error , }
    };
}

DeleteLock!()