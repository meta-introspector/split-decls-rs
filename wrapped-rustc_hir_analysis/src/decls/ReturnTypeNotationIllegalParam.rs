macro_rules! ReturnTypeNotationIllegalParam {
    () => {
        # [derive (Diagnostic)] pub (crate) enum ReturnTypeNotationIllegalParam { # [diag (hir_analysis_return_type_notation_illegal_param_type)] Type { # [primary_span] span : Span , # [label] param_span : Span , } , # [diag (hir_analysis_return_type_notation_illegal_param_const)] Const { # [primary_span] span : Span , # [label] param_span : Span , } , }
    };
}

ReturnTypeNotationIllegalParam!()