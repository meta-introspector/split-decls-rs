// Generated macro for datagram_unsupported (function)
macro_rules! Depcrate_testsdatagram_unsupported {
() => {
// Module: crate::tests
// Provides: {"datagram_unsupported"}
// Dependencies: {}
# [test] fn datagram_unsupported () { let _guard = subscribe () ; let server = ServerConfig { transport : Arc :: new (TransportConfig { datagram_receive_buffer_size : None , .. TransportConfig :: default () }) , .. server_config () } ; let mut pair = Pair :: new (Default :: default () , server) ; let (client_ch , server_ch) = pair . connect () ; assert_matches ! (pair . server_conn_mut (server_ch) . poll () , None) ; assert_matches ! (pair . client_datagrams (client_ch) . max_size () , None) ; match pair . client_datagrams (client_ch) . send (Bytes :: new () , true) { Err (SendDatagramError :: UnsupportedByPeer) => { } Err (e) => panic ! ("unexpected error: {e}") , Ok (_) => panic ! ("unexpected success") , } }
};
}
