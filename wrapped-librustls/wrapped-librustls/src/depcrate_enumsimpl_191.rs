// Generated macro for impl_191 (impl)
macro_rules! Depcrate_enumsimpl_191 {
() => {
// Module: crate::enums
// Provides: {"impl_191"}
// Dependencies: {}
impl From < HandshakeKind > for rustls_handshake_kind { fn from (kind : HandshakeKind) -> Self { match kind { HandshakeKind :: Full => Self :: Full , HandshakeKind :: FullWithHelloRetryRequest => Self :: FullWithHelloRetryRequest , HandshakeKind :: Resumed => Self :: Resumed , } } }
};
}
