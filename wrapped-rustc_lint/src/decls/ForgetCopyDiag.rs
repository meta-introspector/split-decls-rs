macro_rules! deps {
    () => {
        UseLetUnderscoreIgnoreSuggestion!();
    };
}

macro_rules! ForgetCopyDiag {
    () => {
        deps!();
        # [derive (LintDiagnostic)] # [diag (lint_forgetting_copy_types)] pub (crate) struct ForgetCopyDiag < 'a > { pub arg_ty : Ty < 'a > , # [label] pub label : Span , # [subdiagnostic] pub sugg : UseLetUnderscoreIgnoreSuggestion , }
    };
}

ForgetCopyDiag!()