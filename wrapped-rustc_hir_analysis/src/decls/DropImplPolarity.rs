macro_rules! DropImplPolarity {
    () => {
        # [derive (Diagnostic)] pub (crate) enum DropImplPolarity { # [diag (hir_analysis_drop_impl_negative)] Negative { # [primary_span] span : Span , } , # [diag (hir_analysis_drop_impl_reservation)] Reservation { # [primary_span] span : Span , } , }
    };
}

DropImplPolarity!()