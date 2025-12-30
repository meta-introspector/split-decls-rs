// Generated macro for impl_64 (impl)
macro_rules! Depcrateimpl_64 {
() => {
// Module: crate
// Provides: {"impl_64"}
// Dependencies: {}
impl From < openssl :: error :: ErrorStack > for KeySerializationError { fn from (e : openssl :: error :: ErrorStack) -> KeySerializationError { KeySerializationError :: OpenSSL (e) } }
};
}
