// Generated macro for AttributeParseErrorReason (enum)
macro_rules! Depcrate_session_diagnosticsAttributeParseErrorReason {
() => {
// Module: crate::session_diagnostics
// Provides: {"AttributeParseErrorReason"}
// Dependencies: {}
pub (crate) enum AttributeParseErrorReason < 'a > { ExpectedNoArgs , ExpectedStringLiteral { byte_string : Option < Span > , } , ExpectedIntegerLiteral , ExpectedAtLeastOneArgument , ExpectedSingleArgument , ExpectedList , UnexpectedLiteral , ExpectedNameValue (Option < Symbol >) , DuplicateKey (Symbol) , ExpectedSpecificArgument { possibilities : & 'a [Symbol] , strings : bool , # [doc = " Should we tell the user to write a list when they didn't?"] list : bool , } , ExpectedIdentifier , }
};
}
