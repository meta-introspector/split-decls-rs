macro_rules! InvalidGenericReceiverTy {
    () => {
        # [derive (Diagnostic)] # [diag (hir_analysis_invalid_generic_receiver_ty , code = E0801)] # [note] # [help (hir_analysis_invalid_generic_receiver_ty_help)] pub (crate) struct InvalidGenericReceiverTy < 'tcx > { # [primary_span] pub span : Span , pub receiver_ty : Ty < 'tcx > , }
    };
}

InvalidGenericReceiverTy!()