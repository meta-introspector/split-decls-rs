macro_rules! deps {
    () => {
        MissingTraitItemSuggestionNone!();
        MissingTraitItemLabel!();
        MissingTraitItemSuggestion!();
    };
}

macro_rules! MissingTraitItem {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (hir_analysis_missing_trait_item , code = E0046)] pub (crate) struct MissingTraitItem { # [primary_span] # [label] pub span : Span , # [subdiagnostic] pub missing_trait_item_label : Vec < MissingTraitItemLabel > , # [subdiagnostic] pub missing_trait_item : Vec < MissingTraitItemSuggestion > , # [subdiagnostic] pub missing_trait_item_none : Vec < MissingTraitItemSuggestionNone > , pub missing_items_msg : String , }
    };
}

MissingTraitItem!();