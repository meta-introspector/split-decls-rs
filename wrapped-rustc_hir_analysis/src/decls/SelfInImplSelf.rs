macro_rules! SelfInImplSelf {
    () => {
        # [derive (Diagnostic)] # [diag (hir_analysis_self_in_impl_self)] pub (crate) struct SelfInImplSelf { # [primary_span] pub span : MultiSpan , # [note] pub note : () , }
    };
}

SelfInImplSelf!()