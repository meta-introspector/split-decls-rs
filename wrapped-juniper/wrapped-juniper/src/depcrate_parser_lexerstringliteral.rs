// Generated macro for StringLiteral (enum)
macro_rules! Depcrate_parser_lexerStringLiteral {
() => {
// Module: crate::parser::lexer
// Provides: {"StringLiteral"}
// Dependencies: {}
# [doc = " Representation of a raw unparsed [String Value] literal (with quotes included)."] # [doc = ""] # [doc = " [String Value]: https://spec.graphql.org/October2021#sec-String-Value"] # [derive (Clone , Copy , Debug , Display , Eq , PartialEq)] pub enum StringLiteral < 'a > { # [doc = " [Quoted][0] literal (denoted by single quotes `\"`)."] # [doc = ""] # [doc = " [0]: https://spec.graphql.org/October2021#StringCharacter"] Quoted (& 'a str) , # [doc = " [Block][0] literal (denoted by triple quotes `\"\"\"`)."] # [doc = ""] # [doc = " [0]: https://spec.graphql.org/October2021#BlockStringCharacter"] Block (& 'a str) , }
};
}
