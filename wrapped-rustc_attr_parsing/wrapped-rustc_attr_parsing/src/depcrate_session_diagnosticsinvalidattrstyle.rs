// Generated macro for InvalidAttrStyle (struct)
macro_rules! Depcrate_session_diagnosticsInvalidAttrStyle {
() => {
// Module: crate::session_diagnostics
// Provides: {"InvalidAttrStyle"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (attr_parsing_invalid_style)] pub (crate) struct InvalidAttrStyle { pub name : AttrPath , pub is_used_as_inner : bool , # [note] pub target_span : Option < Span > , pub target : Target , }
};
}
