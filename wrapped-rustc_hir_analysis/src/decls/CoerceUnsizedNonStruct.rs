macro_rules! CoerceUnsizedNonStruct {
    () => {
        # [derive (Diagnostic)] # [diag (hir_analysis_coerce_unsized_may , code = E0377)] pub (crate) struct CoerceUnsizedNonStruct { # [primary_span] pub span : Span , pub trait_name : & 'static str , }
    };
}

CoerceUnsizedNonStruct!()