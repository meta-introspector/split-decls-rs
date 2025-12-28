macro_rules! SuggestPtrNullMut {
    () => {
        # [derive (Subdiagnostic)] # [suggestion (hir_typeck_suggest_ptr_null_mut , applicability = "maybe-incorrect" , style = "verbose" , code = "core::ptr::null_mut()")] pub (crate) struct SuggestPtrNullMut { # [primary_span] pub span : Span , }
    };
}

SuggestPtrNullMut!();