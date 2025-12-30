// Generated macro for MacroExpansionParseError (struct)
macro_rules! Depcrate_diagnosticsMacroExpansionParseError {
() => {
// Module: crate::diagnostics
// Provides: {"MacroExpansionParseError"}
// Dependencies: {}
# [derive (Debug , Clone , Eq , PartialEq)] pub struct MacroExpansionParseError { pub node : InFile < SyntaxNodePtr > , pub precise_location : Option < TextRange > , pub errors : Arc < [SyntaxError] > , }
};
}
