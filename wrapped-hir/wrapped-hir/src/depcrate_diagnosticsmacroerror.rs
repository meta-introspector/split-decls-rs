// Generated macro for MacroError (struct)
macro_rules! Depcrate_diagnosticsMacroError {
() => {
// Module: crate::diagnostics
// Provides: {"MacroError"}
// Dependencies: {}
# [derive (Debug , Clone , Eq , PartialEq)] pub struct MacroError { pub node : InFile < SyntaxNodePtr > , pub precise_location : Option < TextRange > , pub message : String , pub error : bool , pub kind : & 'static str , }
};
}
