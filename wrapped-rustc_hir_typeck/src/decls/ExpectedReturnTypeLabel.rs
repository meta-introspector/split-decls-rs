macro_rules! ExpectedReturnTypeLabel {
    () => {
        # [derive (Subdiagnostic)] pub (crate) enum ExpectedReturnTypeLabel < 'tcx > { # [label (hir_typeck_expected_default_return_type)] Unit { # [primary_span] span : Span , } , # [label (hir_typeck_expected_return_type)] Other { # [primary_span] span : Span , expected : Ty < 'tcx > , } , }
    };
}

ExpectedReturnTypeLabel!()