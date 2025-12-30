// Generated macro for Token (struct)
macro_rules! Depcrate_tokenToken {
() => {
// Module: crate::token
// Provides: {"Token"}
// Dependencies: {}
# [doc = " A single token in a C expression."] # [derive (Debug , Clone , PartialEq , Eq)] pub struct Token { # [doc = " The type of this token."] pub kind : Kind , # [doc = " The bytes that make up the token."] pub raw : Box < [u8] > , }
};
}
