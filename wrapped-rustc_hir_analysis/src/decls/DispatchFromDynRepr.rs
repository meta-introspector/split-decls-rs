macro_rules! DispatchFromDynRepr {
    () => {
        # [derive (Diagnostic)] # [diag (hir_analysis_dispatch_from_dyn_repr , code = E0378)] pub (crate) struct DispatchFromDynRepr { # [primary_span] pub span : Span , }
    };
}

DispatchFromDynRepr!()