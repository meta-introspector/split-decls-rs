macro_rules! LifetimesOrBoundsMismatchOnTrait {
    () => {
        # [derive (Diagnostic)] # [diag (hir_analysis_lifetimes_or_bounds_mismatch_on_trait , code = E0195)] pub (crate) struct LifetimesOrBoundsMismatchOnTrait { # [primary_span] # [label] pub span : Span , # [label (hir_analysis_generics_label)] pub generics_span : Option < Span > , # [label (hir_analysis_where_label)] pub where_span : Option < Span > , # [label (hir_analysis_bounds_label)] pub bounds_span : Vec < Span > , pub item_kind : & 'static str , pub ident : Ident , }
    };
}

LifetimesOrBoundsMismatchOnTrait!()