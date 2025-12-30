// Generated macro for impl_1031 (impl)
macro_rules! Depcrate_ssl_errorimpl_1031 {
() => {
// Module: crate::ssl::error
// Provides: {"impl_1031"}
// Dependencies: {}
impl < S > From < ErrorStack > for HandshakeError < S > { fn from (e : ErrorStack) -> HandshakeError < S > { HandshakeError :: SetupFailure (e) } }
};
}
