// Generated macro for DiagnosticSpanMacroExpansion (struct)
macro_rules! Depcrate_format_diagnosticDiagnosticSpanMacroExpansion {
() => {
// Module: crate::format::diagnostic
// Provides: {"DiagnosticSpanMacroExpansion"}
// Dependencies: {}
# [doc = " Macro expansion information associated with a diagnostic."] # [derive (Debug , Clone , PartialEq , Eq , Serialize , Deserialize)] # [cfg_attr (feature = "strict_unstable" , serde (deny_unknown_fields))] # [non_exhaustive] pub struct DiagnosticSpanMacroExpansion < 'a > { # [doc = " span where macro was applied to generate this code; note that"] # [doc = " this may itself derive from a macro (if"] # [doc = " `span.expansion.is_some()`)"] # [serde (borrow)] pub span : DiagnosticSpan < 'a > , # [doc = " name of macro that was applied (e.g., \"foo!\" or \"#[derive(Eq)]\")"] # [serde (borrow)] pub macro_decl_name : CowStr < 'a > , # [doc = " span where macro was defined (if known)"] # [serde (borrow)] pub def_site_span : Option < DiagnosticSpan < 'a > > , }
};
}
