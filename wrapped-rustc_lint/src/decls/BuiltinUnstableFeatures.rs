macro_rules! BuiltinUnstableFeatures {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_builtin_unstable_features)] pub (crate) struct BuiltinUnstableFeatures ;
    };
}

BuiltinUnstableFeatures!()