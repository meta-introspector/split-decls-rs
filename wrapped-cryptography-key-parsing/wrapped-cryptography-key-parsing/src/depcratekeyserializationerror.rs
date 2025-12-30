// Generated macro for KeySerializationError (enum)
macro_rules! DepcrateKeySerializationError {
() => {
// Module: crate
// Provides: {"KeySerializationError"}
// Dependencies: {}
pub enum KeySerializationError { PasswordMustBeUtf8 , Write (asn1 :: WriteError) , OpenSSL (openssl :: error :: ErrorStack) , }
};
}
