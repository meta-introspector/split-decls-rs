macro_rules! deps {
    () => {
        UseLetUnderscoreIgnoreSuggestion!();
    };
}

macro_rules! DropCopyDiag {
    () => {
        deps!();
        # [derive (LintDiagnostic)] # [diag (lint_dropping_copy_types)] pub (crate) struct DropCopyDiag < 'a > { pub arg_ty : Ty < 'a > , # [label] pub label : Span , # [subdiagnostic] pub sugg : UseLetUnderscoreIgnoreSuggestion , }
    };
}

DropCopyDiag!();