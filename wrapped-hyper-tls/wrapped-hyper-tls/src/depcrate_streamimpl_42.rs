// Generated macro for impl_42 (impl)
macro_rules! Depcrate_streamimpl_42 {
() => {
// Module: crate::stream
// Provides: {"impl_42"}
// Dependencies: {}
impl < T > From < TlsStream < TokioIo < T > > > for MaybeHttpsStream < T > { fn from (inner : TlsStream < TokioIo < T > >) -> Self { MaybeHttpsStream :: Https (TokioIo :: new (inner)) } }
};
}
