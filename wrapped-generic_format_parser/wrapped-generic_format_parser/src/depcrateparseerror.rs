// Generated macro for ParseError (struct)
macro_rules! DepcrateParseError {
() => {
// Module: crate
// Provides: {"ParseError"}
// Dependencies: {}
pub struct ParseError { pub description : string :: String , pub note : Option < string :: String > , pub label : string :: String , pub span : InnerSpan , pub secondary_label : Option < (string :: String , InnerSpan) > , pub suggestion : Suggestion , }
};
}
