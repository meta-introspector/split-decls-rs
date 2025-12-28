macro_rules! UndefinedCleanDirtyItem {
    () => {
        # [derive (Diagnostic)] # [diag (incremental_undefined_clean_dirty_assertions_item)] pub (crate) struct UndefinedCleanDirtyItem { # [primary_span] pub span : Span , pub kind : String , }
    };
}

UndefinedCleanDirtyItem!();