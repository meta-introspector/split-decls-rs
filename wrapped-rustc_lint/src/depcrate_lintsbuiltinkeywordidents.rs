// Generated macro for BuiltinKeywordIdents (struct)
macro_rules! Depcrate_lintsBuiltinKeywordIdents {
() => {
// Module: crate::lints
// Provides: {"BuiltinKeywordIdents"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (lint_builtin_keyword_idents)] pub (crate) struct BuiltinKeywordIdents { pub kw : Ident , pub next : Edition , # [suggestion (code = "{prefix}r#{kw}" , applicability = "machine-applicable")] pub suggestion : Span , pub prefix : & 'static str , }
};
}
