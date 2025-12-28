macro_rules! MissingTraitItemUnstable {
    () => {
        # [derive (Diagnostic)] # [diag (hir_analysis_missing_trait_item_unstable , code = E0046)] # [note] pub (crate) struct MissingTraitItemUnstable { # [primary_span] pub span : Span , # [note (hir_analysis_some_note)] pub some_note : bool , # [note (hir_analysis_none_note)] pub none_note : bool , pub missing_item_name : Ident , pub feature : Symbol , pub reason : String , }
    };
}

MissingTraitItemUnstable!();