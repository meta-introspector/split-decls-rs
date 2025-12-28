macro_rules! deps {
    () => {
        AssociatedItemTraitUninferredGenericParamsMultipartSuggestion!();
    };
}

macro_rules! AssociatedItemTraitUninferredGenericParams {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (hir_analysis_associated_type_trait_uninferred_generic_params , code = E0212)] pub (crate) struct AssociatedItemTraitUninferredGenericParams { # [primary_span] pub span : Span , # [suggestion (style = "verbose" , applicability = "maybe-incorrect" , code = "{bound}")] pub inferred_sugg : Option < Span > , pub bound : String , # [subdiagnostic] pub mpart_sugg : Option < AssociatedItemTraitUninferredGenericParamsMultipartSuggestion > , pub what : & 'static str , }
    };
}

AssociatedItemTraitUninferredGenericParams!()