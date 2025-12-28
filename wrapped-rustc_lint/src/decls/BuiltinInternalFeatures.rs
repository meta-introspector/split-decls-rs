macro_rules! BuiltinInternalFeatures {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_builtin_internal_features)] # [note] pub (crate) struct BuiltinInternalFeatures { pub name : Symbol , }
    };
}

BuiltinInternalFeatures!();