macro_rules! SIMDFFIHighlyExperimental {
    () => {
        # [derive (Diagnostic)] # [diag (hir_analysis_simd_ffi_highly_experimental)] # [help] pub (crate) struct SIMDFFIHighlyExperimental { # [primary_span] pub span : Span , pub snip : String , }
    };
}

SIMDFFIHighlyExperimental!()