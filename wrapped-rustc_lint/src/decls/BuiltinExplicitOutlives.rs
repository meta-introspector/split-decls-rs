macro_rules! deps {
    () => {
        BuiltinExplicitOutlivesSuggestion!();
    };
}

macro_rules! BuiltinExplicitOutlives {
    () => {
        deps!();
        # [derive (LintDiagnostic)] # [diag (lint_builtin_explicit_outlives)] pub (crate) struct BuiltinExplicitOutlives { pub count : usize , # [subdiagnostic] pub suggestion : BuiltinExplicitOutlivesSuggestion , }
    };
}

BuiltinExplicitOutlives!();