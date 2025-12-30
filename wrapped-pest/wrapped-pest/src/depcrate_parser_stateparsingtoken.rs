// Generated macro for ParsingToken (enum)
macro_rules! Depcrate_parser_stateParsingToken {
() => {
// Module: crate::parser_state
// Provides: {"ParsingToken"}
// Dependencies: {}
# [derive (Debug , Clone , PartialEq , Eq , Hash , PartialOrd , Ord)] pub enum ParsingToken { Sensitive { token : String } , Insensitive { token : String } , Range { start : char , end : char } , BuiltInRule , }
};
}
