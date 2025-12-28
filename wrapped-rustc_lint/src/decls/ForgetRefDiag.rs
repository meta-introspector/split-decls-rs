macro_rules! deps {
    () => {
        UseLetUnderscoreIgnoreSuggestion!();
    };
}

macro_rules! ForgetRefDiag {
    () => {
        deps!();
        # [derive (LintDiagnostic)] # [diag (lint_forgetting_references)] pub (crate) struct ForgetRefDiag < 'a > { pub arg_ty : Ty < 'a > , # [label] pub label : Span , # [subdiagnostic] pub sugg : UseLetUnderscoreIgnoreSuggestion , }
    };
}

ForgetRefDiag!()