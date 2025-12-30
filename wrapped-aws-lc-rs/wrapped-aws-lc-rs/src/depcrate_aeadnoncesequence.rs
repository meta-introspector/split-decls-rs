// Generated macro for NonceSequence (trait)
macro_rules! Depcrate_aeadNonceSequence {
() => {
// Module: crate::aead
// Provides: {"NonceSequence"}
// Dependencies: {}
# [doc = " A sequences of unique nonces."] # [doc = ""] # [doc = " A given `NonceSequence` must never return the same `Nonce` twice from"] # [doc = " `advance()`."] # [doc = ""] # [doc = " A simple counter is a reasonable (but probably not ideal) `NonceSequence`."] # [doc = ""] # [doc = " Intentionally not `Clone` or `Copy` since cloning would allow duplication"] # [doc = " of the sequence."] pub trait NonceSequence { # [doc = " Returns the next nonce in the sequence."] # [doc = ""] # [doc = " # Errors"] # [doc = " `error::Unspecified` if  \"too many\" nonces have been requested, where how many"] # [doc = " is too many is up to the implementation of `NonceSequence`. An"] # [doc = " implementation may that enforce a maximum number of records are"] # [doc = " sent/received under a key this way. Once `advance()` fails, it must"] # [doc = " fail for all subsequent calls."] fn advance (& mut self) -> Result < Nonce , Unspecified > ; }
};
}
