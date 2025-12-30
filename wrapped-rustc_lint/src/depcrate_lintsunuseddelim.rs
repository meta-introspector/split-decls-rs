// Generated macro for UnusedDelim (struct)
macro_rules! Depcrate_lintsUnusedDelim {
() => {
// Module: crate::lints
// Provides: {"UnusedDelim"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (lint_unused_delim)] pub (crate) struct UnusedDelim < 'a > { pub delim : & 'static str , pub item : & 'a str , # [subdiagnostic] pub suggestion : Option < UnusedDelimSuggestion > , }
};
}
