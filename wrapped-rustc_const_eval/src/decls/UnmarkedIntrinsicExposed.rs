macro_rules! UnmarkedIntrinsicExposed {
    () => {
        # [derive (Diagnostic)] # [diag (const_eval_unmarked_intrinsic_exposed)] # [help] pub (crate) struct UnmarkedIntrinsicExposed { # [primary_span] pub span : Span , pub def_path : String , }
    };
}

UnmarkedIntrinsicExposed!();