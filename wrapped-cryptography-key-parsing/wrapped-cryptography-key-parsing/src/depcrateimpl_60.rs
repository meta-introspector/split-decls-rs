// Generated macro for impl_60 (impl)
macro_rules! Depcrateimpl_60 {
() => {
// Module: crate
// Provides: {"impl_60"}
// Dependencies: {}
impl From < openssl :: error :: ErrorStack > for KeyParsingError { fn from (e : openssl :: error :: ErrorStack) -> KeyParsingError { KeyParsingError :: OpenSSL (e) } }
};
}
