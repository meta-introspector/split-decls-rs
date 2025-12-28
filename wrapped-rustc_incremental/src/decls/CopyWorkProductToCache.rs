macro_rules! CopyWorkProductToCache {
    () => {
        # [derive (Diagnostic)] # [diag (incremental_copy_workproduct_to_cache)] pub (crate) struct CopyWorkProductToCache < 'a > { pub from : & 'a Path , pub to : & 'a Path , pub err : std :: io :: Error , }
    };
}

CopyWorkProductToCache!()