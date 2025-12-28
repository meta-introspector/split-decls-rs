macro_rules! NonConstImplNote {
    () => {
        # [derive (Subdiagnostic)] # [note (const_eval_non_const_impl)] pub (crate) struct NonConstImplNote { # [primary_span] pub span : Span , }
    };
}

NonConstImplNote!();