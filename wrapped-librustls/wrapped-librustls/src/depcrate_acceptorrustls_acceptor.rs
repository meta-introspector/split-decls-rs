// Generated macro for rustls_acceptor (struct)
macro_rules! Depcrate_acceptorrustls_acceptor {
() => {
// Module: crate::acceptor
// Provides: {"rustls_acceptor"}
// Dependencies: {}
# [doc = " A buffer and parser for ClientHello bytes."] # [doc = ""] # [doc = " This allows reading ClientHello before choosing a rustls_server_config."] # [doc = ""] # [doc = " It's useful when the server config will be based on parameters in the"] # [doc = " ClientHello: server name indication (SNI), ALPN protocols, signature"] # [doc = " schemes, and cipher suites."] # [doc = ""] # [doc = " In particular, if a server wants to do some potentially expensive work"] # [doc = " to load a certificate for a given hostname, rustls_acceptor allows doing"] # [doc = " that asynchronously, as opposed to rustls_server_config_builder_set_hello_callback(),"] # [doc = " which doesn't work well for asynchronous I/O."] # [doc = ""] # [doc = " The general flow is:"] # [doc = "  - rustls_acceptor_new()"] # [doc = "  - Loop:"] # [doc = "    - Read bytes from the network it with rustls_acceptor_read_tls()."] # [doc = "    - If successful, parse those bytes with rustls_acceptor_accept()."] # [doc = "    - If that returns RUSTLS_RESULT_ACCEPTOR_NOT_READY, continue."] # [doc = "    - Otherwise, break."] # [doc = "  - If rustls_acceptor_accept() returned RUSTLS_RESULT_OK:"] # [doc = "    - Examine the resulting rustls_accepted."] # [doc = "    - Create or select a rustls_server_config."] # [doc = "    - Call rustls_accepted_into_connection()."] # [doc = "  - Otherwise, there was a problem with the ClientHello data and the"] # [doc = "    connection should be rejected."] pub struct rustls_acceptor { _private : [u8 ; 0] , }
};
}
