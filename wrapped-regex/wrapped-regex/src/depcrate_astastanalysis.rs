// Generated macro for AstAnalysis (struct)
macro_rules! Depcrate_astAstAnalysis {
() => {
// Module: crate::ast
// Provides: {"AstAnalysis"}
// Dependencies: {}
# [doc = " The results of analyzing AST of a regular expression (e.g., for supporting"] # [doc = " smart case)."] # [derive (Clone , Debug)] pub (crate) struct AstAnalysis { # [doc = " True if and only if a literal uppercase character occurs in the regex."] any_uppercase : bool , # [doc = " True if and only if the regex contains any literal at all."] any_literal : bool , }
};
}
