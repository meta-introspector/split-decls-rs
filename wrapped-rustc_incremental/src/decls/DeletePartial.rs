macro_rules! DeletePartial {
    () => {
        # [derive (Diagnostic)] # [diag (incremental_delete_partial)] pub (crate) struct DeletePartial < 'a > { pub path : & 'a Path , pub err : std :: io :: Error , }
    };
}

DeletePartial!();