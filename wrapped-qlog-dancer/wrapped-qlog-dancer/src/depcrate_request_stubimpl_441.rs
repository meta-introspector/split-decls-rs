// Generated macro for impl_441 (impl)
macro_rules! Depcrate_request_stubimpl_441 {
() => {
// Module: crate::request_stub
// Provides: {"impl_441"}
// Dependencies: {}
impl HttpRequestStub { fn client_deltas (& self) -> ClientDeltaStrings { match self . at_client_deltas { Some (d) => ClientDeltaStrings { discovery_tx_hdr : d . discover_tx_hdr . to_string () , tx_hdr_rx_hdr : d . tx_hdr_rx_hdr . to_string () , tx_hdr_rx_first_data : d . tx_hdr_rx_first_data . to_string () , tx_hdr_rx_last_data : d . tx_hdr_rx_last_data . to_string () , download_time_d2d : d . rx_first_data_rx_last_data . to_string () , download_time_h2d : d . tx_hdr_rx_last_data . to_string () , upload_time : d . tx_first_data_tx_last_data . to_string () , } , None => ClientDeltaStrings :: default () , } } fn server_deltas (& self) -> (String , String , String , String) { match self . at_server_deltas { Some (d) => (d . rx_hdr_tx_hdr . to_string () , d . rx_hdr_tx_first_data . to_string () , d . rx_hdr_tx_last_data . to_string () , d . tx_first_data_tx_last_data . to_string () ,) , None => { let na = NaOption :: < u8 > :: new (None) ; (na . to_string () , na . to_string () , na . to_string () , na . to_string () ,) } , } } }
};
}
