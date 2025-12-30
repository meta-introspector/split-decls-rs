// Generated macro for BuiltinIncompleteFeatures (struct)
macro_rules! Depcrate_lintsBuiltinIncompleteFeatures {
() => {
// Module: crate::lints
// Provides: {"BuiltinIncompleteFeatures"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (lint_builtin_incomplete_features)] pub (crate) struct BuiltinIncompleteFeatures { pub name : Symbol , # [subdiagnostic] pub note : Option < BuiltinFeatureIssueNote > , # [subdiagnostic] pub help : Option < BuiltinIncompleteFeaturesHelp > , }
};
}
