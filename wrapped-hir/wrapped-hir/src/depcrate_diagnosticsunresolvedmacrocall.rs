// Generated macro for UnresolvedMacroCall (struct)
macro_rules! Depcrate_diagnosticsUnresolvedMacroCall {
() => {
// Module: crate::diagnostics
// Provides: {"UnresolvedMacroCall"}
// Dependencies: {}
# [derive (Debug , Clone , Eq , PartialEq)] pub struct UnresolvedMacroCall { pub macro_call : InFile < SyntaxNodePtr > , pub precise_location : Option < TextRange > , pub path : ModPath , pub is_bang : bool , }
};
}
