macro_rules! CantDereference {
    () => {
        # [derive (Diagnostic)] # [diag (hir_typeck_cant_dereference , code = E0614)] pub (crate) struct CantDereference < 'tcx > { # [primary_span] # [label (hir_typeck_cant_dereference_label)] pub (crate) span : Span , pub (crate) ty : Ty < 'tcx > , }
    };
}

CantDereference!();