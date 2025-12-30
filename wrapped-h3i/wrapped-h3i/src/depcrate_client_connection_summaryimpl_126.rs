// Generated macro for impl_126 (impl)
macro_rules! Depcrate_client_connection_summaryimpl_126 {
() => {
// Module: crate::client::connection_summary
// Provides: {"impl_126"}
// Dependencies: {}
impl core :: fmt :: Debug for ConnectionCloseDetails { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . debug_struct ("ConnectionCloseDetails") . field ("peer_error" , & self . peer_error) . field ("local_error" , & self . local_error) . field ("timed_out" , & self . timed_out) . finish () } }
};
}
