// Generated macro for ParseError (enum)
macro_rules! DepcrateParseError {
() => {
// Module: crate
// Provides: {"ParseError"}
// Dependencies: {}
# [derive (Debug , PartialEq , Eq , Clone)] pub enum ParseError { UnexpectedToken (Box < str >) , Expected (Box < str >) , InvalidRepeat , RepetitionEmptyTokenTree , }
};
}
