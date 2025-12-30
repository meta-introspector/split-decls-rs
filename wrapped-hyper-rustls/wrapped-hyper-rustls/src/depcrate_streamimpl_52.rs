// Generated macro for impl_52 (impl)
macro_rules! Depcrate_streamimpl_52 {
() => {
// Module: crate::stream
// Provides: {"impl_52"}
// Dependencies: {}
impl < T > From < TlsStream < TokioIo < T > > > for MaybeHttpsStream < T > { fn from (inner : TlsStream < TokioIo < T > >) -> Self { Self :: Https (TokioIo :: new (inner)) } }
};
}
