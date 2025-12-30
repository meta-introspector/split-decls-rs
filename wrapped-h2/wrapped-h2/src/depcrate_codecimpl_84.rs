// Generated macro for impl_84 (impl)
macro_rules! Depcrate_codecimpl_84 {
() => {
// Module: crate::codec
// Provides: {"impl_84"}
// Dependencies: {}
impl < T > From < T > for Codec < T , bytes :: Bytes > where T : AsyncRead + AsyncWrite + Unpin , { fn from (src : T) -> Self { Self :: new (src) } }
};
}
