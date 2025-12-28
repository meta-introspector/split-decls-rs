macro_rules! deps {
    () => {
        UseLetUnderscoreIgnoreSuggestion!();
    };
}

macro_rules! DropRefDiag {
    () => {
        deps!();
        # [derive (LintDiagnostic)] # [diag (lint_dropping_references)] pub (crate) struct DropRefDiag < 'a > { pub arg_ty : Ty < 'a > , # [label] pub label : Span , # [subdiagnostic] pub sugg : UseLetUnderscoreIgnoreSuggestion , }
    };
}

DropRefDiag!()