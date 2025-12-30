// Generated macro for SingleUseLifetime (struct)
macro_rules! Depcrate_lintsSingleUseLifetime {
() => {
// Module: crate::lints
// Provides: {"SingleUseLifetime"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (lint_single_use_lifetime)] pub (crate) struct SingleUseLifetime { # [label (lint_label_param)] pub param_span : Span , # [label (lint_label_use)] pub use_span : Span , # [subdiagnostic] pub suggestion : Option < SingleUseLifetimeSugg > , pub ident : Ident , }
};
}
