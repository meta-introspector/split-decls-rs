// Generated macro for UnusedImports (struct)
macro_rules! Depcrate_lintsUnusedImports {
() => {
// Module: crate::lints
// Provides: {"UnusedImports"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (lint_unused_imports)] pub (crate) struct UnusedImports { # [subdiagnostic] pub sugg : UnusedImportsSugg , # [help] pub test_module_span : Option < Span > , pub span_snippets : DiagArgValue , pub num_snippets : usize , }
};
}
