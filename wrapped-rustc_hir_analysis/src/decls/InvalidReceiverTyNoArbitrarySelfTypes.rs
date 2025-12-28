macro_rules! InvalidReceiverTyNoArbitrarySelfTypes {
    () => {
        # [derive (Diagnostic)] # [diag (hir_analysis_invalid_receiver_ty_no_arbitrary_self_types , code = E0307)] # [note] # [help (hir_analysis_invalid_receiver_ty_help_no_arbitrary_self_types)] pub (crate) struct InvalidReceiverTyNoArbitrarySelfTypes < 'tcx > { # [primary_span] pub span : Span , pub receiver_ty : Ty < 'tcx > , }
    };
}

InvalidReceiverTyNoArbitrarySelfTypes!()