// Generated macro for ScalarToken (enum)
macro_rules! Depcrate_parser_lexerScalarToken {
() => {
// Module: crate::parser::lexer
// Provides: {"ScalarToken"}
// Dependencies: {}
# [doc = " Representation of a raw unparsed scalar value literal."] # [doc = ""] # [doc = " This is only used for tagging how the lexer has interpreted a value literal"] # [expect (missing_docs , reason = "self-explanatory")] # [derive (Clone , Copy , Debug , Display , Eq , PartialEq)] pub enum ScalarToken < 'a > { String (StringLiteral < 'a >) , Float (& 'a str) , Int (& 'a str) , }
};
}
