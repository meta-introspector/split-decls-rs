macro_rules! deps {
    () => {
        UndroppedManuallyDropsSuggestion!();
    };
}

macro_rules! UndroppedManuallyDropsDiag {
    () => {
        deps!();
        # [derive (LintDiagnostic)] # [diag (lint_undropped_manually_drops)] pub (crate) struct UndroppedManuallyDropsDiag < 'a > { pub arg_ty : Ty < 'a > , # [label] pub label : Span , # [subdiagnostic] pub suggestion : UndroppedManuallyDropsSuggestion , }
    };
}

UndroppedManuallyDropsDiag!();