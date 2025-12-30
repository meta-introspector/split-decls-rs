// Generated macro for MaybeHttpsStream (enum)
macro_rules! Depcrate_streamMaybeHttpsStream {
() => {
// Module: crate::stream
// Provides: {"MaybeHttpsStream"}
// Dependencies: {}
# [doc = " A stream that might be protected with TLS."] # [allow (clippy :: large_enum_variant)] pub enum MaybeHttpsStream < T > { # [doc = " A stream over plain text."] Http (T) , # [doc = " A stream protected with TLS."] Https (TokioIo < TlsStream < TokioIo < T > > >) , }
};
}
