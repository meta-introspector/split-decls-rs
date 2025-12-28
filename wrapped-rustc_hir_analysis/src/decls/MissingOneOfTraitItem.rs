macro_rules! MissingOneOfTraitItem {
    () => {
        # [derive (Diagnostic)] # [diag (hir_analysis_missing_one_of_trait_item , code = E0046)] pub (crate) struct MissingOneOfTraitItem { # [primary_span] # [label] pub span : Span , # [note] pub note : Option < Span > , pub missing_items_msg : String , }
    };
}

MissingOneOfTraitItem!();