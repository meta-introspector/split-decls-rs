macro_rules! SupertraitAsDerefTargetLabel {
    () => {
        # [derive (Subdiagnostic)] # [label (lint_label2)] pub (crate) struct SupertraitAsDerefTargetLabel { # [primary_span] pub label : Span , }
    };
}

SupertraitAsDerefTargetLabel!();