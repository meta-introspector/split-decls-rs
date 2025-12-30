// Generated macro for impl_106 (impl)
macro_rules! Depcrate_datastoreimpl_106 {
() => {
// Module: crate::datastore
// Provides: {"impl_106"}
// Dependencies: {}
impl Display for QuicStreamReset { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "offset={}, code={}, {}" , self . offset , self . quic_rst_stream_error , self . quic_rst_stream_error_friendly . as_deref () . unwrap_or ("")) } }
};
}
