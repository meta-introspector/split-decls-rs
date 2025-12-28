macro_rules! UndefinedCleanDirty {
    () => {
        # [derive (Diagnostic)] # [diag (incremental_undefined_clean_dirty_assertions)] pub (crate) struct UndefinedCleanDirty { # [primary_span] pub span : Span , pub kind : String , }
    };
}

UndefinedCleanDirty!()