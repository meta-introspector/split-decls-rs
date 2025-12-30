// Generated macro for impl_104 (impl)
macro_rules! Depcrate_datastoreimpl_104 {
() => {
// Module: crate::datastore
// Provides: {"impl_104"}
// Dependencies: {}
impl Display for QuicStreamStopSending { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "code={}, {}" , self . quic_rst_stream_error , self . quic_rst_stream_error_friendly . as_deref () . unwrap_or ("")) } }
};
}
