// Generated macro for Error (enum)
macro_rules! Depcrate_errorError {
() => {
// Module: crate::error
// Provides: {"Error"}
// Dependencies: {}
# [derive (Debug , Copy , Clone , Eq , PartialEq)] pub enum Error { # [doc = " The signature doesn't verify."] SignatureMismatch , # [doc = " A weak public key was used."] WeakPublicKey , # [doc = " The public key is invalid."] InvalidPublicKey , # [doc = " The secret key is invalid."] InvalidSecretKey , # [doc = " The signature is invalid."] InvalidSignature , # [doc = " The seed doesn't have the expected length."] InvalidSeed , # [doc = " The blind doesn't have the expected length."] InvalidBlind , # [doc = " The noise doesn't have the expected length."] InvalidNoise , # [doc = " Parse error"] ParseError , # [doc = " Non-canonical encoding"] NonCanonical , }
};
}
