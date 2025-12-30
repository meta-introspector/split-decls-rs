// Generated macro for InvalidToken (struct)
macro_rules! Depcrate_errInvalidToken {
() => {
// Module: crate::err
// Provides: {"InvalidToken"}
// Dependencies: {}
# [doc = " An error signaling that a different kind of token was expected. Returned by"] # [doc = " the various `TryFrom` impls."] # [derive (Debug , Clone , Copy)] pub struct InvalidToken { pub (crate) expected : TokenKind , pub (crate) actual : TokenKind , pub (crate) span : Span , }
};
}
