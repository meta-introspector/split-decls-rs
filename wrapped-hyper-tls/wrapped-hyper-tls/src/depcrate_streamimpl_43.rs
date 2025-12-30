// Generated macro for impl_43 (impl)
macro_rules! Depcrate_streamimpl_43 {
() => {
// Module: crate::stream
// Provides: {"impl_43"}
// Dependencies: {}
impl < T > From < TokioIo < TlsStream < TokioIo < T > > > > for MaybeHttpsStream < T > { fn from (inner : TokioIo < TlsStream < TokioIo < T > > >) -> Self { MaybeHttpsStream :: Https (inner) } }
};
}
