// Generated macro for State (enum)
macro_rules! Depcrate_tokenizer_statesState {
() => {
// Module: crate::tokenizer::states
// Provides: {"State"}
// Dependencies: {}
# [derive (PartialEq , Eq , PartialOrd , Ord , Copy , Clone , Hash , Debug)] pub enum State { Data , Plaintext , TagOpen , EndTagOpen , TagName , RawData (RawKind) , RawLessThanSign (RawKind) , RawEndTagOpen (RawKind) , RawEndTagName (RawKind) , ScriptDataEscapeStart (ScriptEscapeKind) , ScriptDataEscapeStartDash , ScriptDataEscapedDash (ScriptEscapeKind) , ScriptDataEscapedDashDash (ScriptEscapeKind) , ScriptDataDoubleEscapeEnd , BeforeAttributeName , AttributeName , AfterAttributeName , BeforeAttributeValue , AttributeValue (AttrValueKind) , AfterAttributeValueQuoted , SelfClosingStartTag , BogusComment , MarkupDeclarationOpen , CommentStart , CommentStartDash , Comment , CommentEndDash , CommentEnd , CommentEndBang , Doctype , BeforeDoctypeName , DoctypeName , AfterDoctypeName , AfterDoctypeKeyword (DoctypeIdKind) , BeforeDoctypeIdentifier (DoctypeIdKind) , DoctypeIdentifierDoubleQuoted (DoctypeIdKind) , DoctypeIdentifierSingleQuoted (DoctypeIdKind) , AfterDoctypeIdentifier (DoctypeIdKind) , BetweenDoctypePublicAndSystemIdentifiers , BogusDoctype , CdataSection , Quiescent , }
};
}
