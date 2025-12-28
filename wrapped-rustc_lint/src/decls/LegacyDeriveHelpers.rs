macro_rules! LegacyDeriveHelpers {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_legacy_derive_helpers)] pub (crate) struct LegacyDeriveHelpers { # [label] pub span : Span , }
    };
}

LegacyDeriveHelpers!();