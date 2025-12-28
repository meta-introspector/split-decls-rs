macro_rules! TailExprDropOrder {
    () => {
        # [derive (LintDiagnostic)] # [diag (borrowck_tail_expr_drop_order)] pub (crate) struct TailExprDropOrder { # [label] pub borrowed : Span , }
    };
}

TailExprDropOrder!();