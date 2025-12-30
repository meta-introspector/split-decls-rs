// Generated macro for Token (enum)
macro_rules! Depcrate_tokenizer_interfaceToken {
() => {
// Module: crate::tokenizer::interface
// Provides: {"Token"}
// Dependencies: {}
# [derive (PartialEq , Eq , Debug)] pub enum Token { DoctypeToken (Doctype) , TagToken (Tag) , CommentToken (StrTendril) , CharacterTokens (StrTendril) , NullCharacterToken , EOFToken , ParseError (Cow < 'static , str >) , }
};
}
