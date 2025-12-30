// Generated macro for new_session (function)
macro_rules! Depcrate_tlsnew_session {
() => {
// Module: crate::tls
// Provides: {"new_session"}
// Dependencies: {}
extern "C" fn new_session (ssl : * mut SSL , session : * mut SSL_SESSION) -> c_int { let ex_data = match ExData :: from_ssl_ptr (ssl) { Some (v) => v , None => return 0 , } ; let handshake = Handshake :: new (ssl) ; let peer_params = handshake . quic_transport_params () ; let session_bytes = match get_session_bytes (session) { Ok (v) => v , Err (_) => return 0 , } ; let mut buffer = Vec :: with_capacity (8 + peer_params . len () + 8 + session_bytes . len ()) ; let session_bytes_len = session_bytes . len () as u64 ; if buffer . write (& session_bytes_len . to_be_bytes ()) . is_err () { std :: mem :: forget (handshake) ; return 0 ; } if buffer . write (& session_bytes) . is_err () { std :: mem :: forget (handshake) ; return 0 ; } let peer_params_len = peer_params . len () as u64 ; if buffer . write (& peer_params_len . to_be_bytes ()) . is_err () { std :: mem :: forget (handshake) ; return 0 ; } if buffer . write (peer_params) . is_err () { std :: mem :: forget (handshake) ; return 0 ; } * ex_data . session = Some (buffer) ; std :: mem :: forget (handshake) ; 0 }
};
}
