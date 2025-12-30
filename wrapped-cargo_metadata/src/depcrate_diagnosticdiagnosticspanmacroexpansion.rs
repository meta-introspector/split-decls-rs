// Generated macro for DiagnosticSpanMacroExpansion (struct)
macro_rules! Depcrate_diagnosticDiagnosticSpanMacroExpansion {
() => {
// Module: crate::diagnostic
// Provides: {"DiagnosticSpanMacroExpansion"}
// Dependencies: {}
# [doc = " Macro expansion information associated with a diagnostic."] # [derive (Debug , Clone , Serialize , Deserialize , PartialEq , Eq , Hash)] # [cfg_attr (feature = "builder" , derive (Builder))] # [non_exhaustive] # [cfg_attr (feature = "builder" , builder (pattern = "owned" , setter (into)))] pub struct DiagnosticSpanMacroExpansion { # [doc = " span where macro was applied to generate this code; note that"] # [doc = " this may itself derive from a macro (if"] # [doc = " `span.expansion.is_some()`)"] pub span : DiagnosticSpan , # [doc = " name of macro that was applied (e.g., \"foo!\" or \"#[derive(Eq)]\")"] pub macro_decl_name : String , # [doc = " span where macro was defined (if known)"] pub def_site_span : Option < DiagnosticSpan > , }
};
}
