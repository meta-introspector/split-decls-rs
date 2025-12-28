macro_rules! DispatchFromDynZST {
    () => {
        # [derive (Diagnostic)] # [diag (hir_analysis_dispatch_from_dyn_zst , code = E0378)] # [note] pub (crate) struct DispatchFromDynZST < 'a > { # [primary_span] pub span : Span , pub name : Ident , pub ty : Ty < 'a > , }
    };
}

DispatchFromDynZST!();