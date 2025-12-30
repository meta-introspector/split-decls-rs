// Generated macro for RefOfMutStatic (struct)
macro_rules! Depcrate_lintsRefOfMutStatic {
() => {
// Module: crate::lints
// Provides: {"RefOfMutStatic"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (lint_static_mut_refs_lint)] pub (crate) struct RefOfMutStatic < 'a > { # [label] pub span : Span , # [subdiagnostic] pub sugg : Option < MutRefSugg > , pub shared_label : & 'a str , # [note (lint_shared_note)] pub shared_note : bool , # [note (lint_mut_note)] pub mut_note : bool , }
};
}
