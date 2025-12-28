macro_rules! OpaqueCapturesHigherRankedLifetime {
    () => {
        # [derive (Diagnostic)] # [diag (hir_analysis_opaque_captures_higher_ranked_lifetime , code = E0657)] pub (crate) struct OpaqueCapturesHigherRankedLifetime { # [primary_span] pub span : Span , # [label] pub label : Option < Span > , # [note] pub decl_span : Span , pub bad_place : & 'static str , }
    };
}

OpaqueCapturesHigherRankedLifetime!()