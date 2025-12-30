// Generated macro for ConfusableIdentifierPair (struct)
macro_rules! Depcrate_lintsConfusableIdentifierPair {
() => {
// Module: crate::lints
// Provides: {"ConfusableIdentifierPair"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (lint_confusable_identifier_pair)] pub (crate) struct ConfusableIdentifierPair { pub existing_sym : Symbol , pub sym : Symbol , # [label (lint_other_use)] pub label : Span , # [label (lint_current_use)] pub main_label : Span , }
};
}
