macro_rules! SpanUseEqCtxtDiag {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_span_use_eq_ctxt)] pub (crate) struct SpanUseEqCtxtDiag ;
    };
}

SpanUseEqCtxtDiag!();