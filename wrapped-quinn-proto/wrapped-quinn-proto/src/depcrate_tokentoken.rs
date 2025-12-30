// Generated macro for Token (struct)
macro_rules! Depcrate_tokenToken {
() => {
// Module: crate::token
// Provides: {"Token"}
// Dependencies: {}
# [doc = " Retry or validation token"] pub (crate) struct Token { # [doc = " Content that is encrypted from the client"] pub (crate) payload : TokenPayload , # [doc = " Randomly generated value, which must be unique, and is visible to the client"] nonce : u128 , }
};
}
