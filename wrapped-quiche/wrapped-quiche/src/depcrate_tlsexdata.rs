// Generated macro for ExData (struct)
macro_rules! Depcrate_tlsExData {
() => {
// Module: crate::tls
// Provides: {"ExData"}
// Dependencies: {}
pub struct ExData < 'a > { pub application_protos : & 'a Vec < Vec < u8 > > , pub crypto_ctx : & 'a mut [packet :: CryptoContext ; packet :: Epoch :: count ()] , pub session : & 'a mut Option < Vec < u8 > > , pub local_error : & 'a mut Option < ConnectionError > , pub keylog : Option < & 'a mut Box < dyn Write + Send + Sync > > , pub trace_id : & 'a str , pub local_transport_params : crate :: TransportParams , pub recovery_config : crate :: recovery :: RecoveryConfig , pub tx_cap_factor : f64 , pub pmtud : Option < bool > , pub is_server : bool , }
};
}
