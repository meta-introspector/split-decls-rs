// Generated macro for KeyPair (struct)
macro_rules! DepcrateKeyPair {
() => {
// Module: crate
// Provides: {"KeyPair"}
// Dependencies: {}
# [doc = " An ML-DSA key pair"] pub struct KeyPair < P : MlDsaParams > { # [doc = " The signing key of the key pair"] signing_key : SigningKey < P > , # [doc = " The verifying key of the key pair"] verifying_key : VerifyingKey < P > , # [doc = " The seed this signing key was derived from"] seed : B32 , }
};
}
