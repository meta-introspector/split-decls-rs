// Generated macro for RedundantImport (struct)
macro_rules! Depcrate_lintsRedundantImport {
() => {
// Module: crate::lints
// Provides: {"RedundantImport"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (lint_redundant_import)] pub (crate) struct RedundantImport { # [subdiagnostic] pub subs : Vec < RedundantImportSub > , pub ident : Ident , }
};
}
