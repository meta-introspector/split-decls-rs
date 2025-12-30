// Generated macro for SslStream (struct)
macro_rules! Depcrate_sslSslStream {
() => {
// Module: crate::ssl
// Provides: {"SslStream"}
// Dependencies: {}
# [doc = " A TLS session over a stream."] pub struct SslStream < S > { ssl : ManuallyDrop < Ssl > , method : ManuallyDrop < BioMethod > , _p : PhantomData < S > , }
};
}
