// Generated macro for impl_1224 (impl)
macro_rules! Depcrate_sslimpl_1224 {
() => {
// Module: crate::ssl
// Provides: {"impl_1224"}
// Dependencies: {}
impl < S > MidHandshakeSslStream < S > { # [doc = " Returns a shared reference to the inner stream."] pub fn get_ref (& self) -> & S { self . stream . get_ref () } # [doc = " Returns a mutable reference to the inner stream."] pub fn get_mut (& mut self) -> & mut S { self . stream . get_mut () } # [doc = " Returns a shared reference to the `Ssl` of the stream."] pub fn ssl (& self) -> & SslRef { self . stream . ssl () } # [doc = " Returns the underlying error which interrupted this handshake."] pub fn error (& self) -> & Error { & self . error } # [doc = " Consumes `self`, returning its error."] pub fn into_error (self) -> Error { self . error } }
};
}
