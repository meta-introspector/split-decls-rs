macro_rules! MissingTraitItemLabel {
    () => {
        # [derive (Subdiagnostic)] # [label (hir_analysis_missing_trait_item_label)] pub (crate) struct MissingTraitItemLabel { # [primary_span] pub span : Span , pub item : Symbol , }
    };
}

MissingTraitItemLabel!();