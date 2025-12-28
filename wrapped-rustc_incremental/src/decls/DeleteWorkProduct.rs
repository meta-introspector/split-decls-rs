macro_rules! DeleteWorkProduct {
    () => {
        # [derive (Diagnostic)] # [diag (incremental_delete_workproduct)] pub (crate) struct DeleteWorkProduct < 'a > { pub path : & 'a Path , pub err : std :: io :: Error , }
    };
}

DeleteWorkProduct!();