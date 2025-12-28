macro_rules! CoerceSameStruct {
    () => {
        # [derive (Diagnostic)] # [diag (hir_analysis_coerce_unsized_may , code = E0377)] pub (crate) struct CoerceSameStruct { # [primary_span] pub span : Span , pub trait_name : & 'static str , # [note (hir_analysis_coercion_between_struct_same_note)] pub note : bool , pub source_path : String , pub target_path : String , }
    };
}

CoerceSameStruct!()