macro_rules! AssocItemNotFoundLabel {
    () => {
        # [derive (Subdiagnostic)] pub (crate) enum AssocItemNotFoundLabel < 'a > { # [label (hir_analysis_assoc_item_not_found_label)] NotFound { # [primary_span] span : Span , } , # [label (hir_analysis_assoc_item_not_found_found_in_other_trait_label)] FoundInOtherTrait { # [primary_span] span : Span , assoc_kind : & 'static str , trait_name : & 'a str , suggested_name : Symbol , identically_named : bool , } , }
    };
}

AssocItemNotFoundLabel!();