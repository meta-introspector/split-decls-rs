macro_rules! deps {
    () => {
        AssocKindMismatchWrapInBracesSugg!();
    };
}

macro_rules! AssocKindMismatch {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (hir_analysis_assoc_kind_mismatch)] pub (crate) struct AssocKindMismatch { # [primary_span] # [label] pub span : Span , pub expected : & 'static str , pub got : & 'static str , # [label (hir_analysis_expected_because_label)] pub expected_because_label : Option < Span > , pub assoc_kind : & 'static str , # [note] pub def_span : Span , # [label (hir_analysis_bound_on_assoc_const_label)] pub bound_on_assoc_const_label : Option < Span > , # [subdiagnostic] pub wrap_in_braces_sugg : Option < AssocKindMismatchWrapInBracesSugg > , }
    };
}

AssocKindMismatch!();