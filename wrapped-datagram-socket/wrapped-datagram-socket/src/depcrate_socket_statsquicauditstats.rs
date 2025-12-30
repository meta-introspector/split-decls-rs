// Generated macro for QuicAuditStats (struct)
macro_rules! Depcrate_socket_statsQuicAuditStats {
() => {
// Module: crate::socket_stats
// Provides: {"QuicAuditStats"}
// Dependencies: {}
# [derive (Debug)] pub struct QuicAuditStats { # [doc = " A transport-level connection error code received from the client."] recvd_conn_close_transport_error_code : AtomicI64 , # [doc = " A transport-level connection error code sent to the client."] sent_conn_close_transport_error_code : AtomicI64 , # [doc = " An application-level connection error code received from the client."] recvd_conn_close_application_error_code : AtomicI64 , # [doc = " An application-level connection error code sent to the client."] sent_conn_close_application_error_code : AtomicI64 , # [doc = " Time taken for the QUIC handshake in microseconds."] transport_handshake_duration_us : AtomicI64 , # [doc = " The start time of the handshake."] transport_handshake_start : Arc < RwLock < Option < SystemTime > > > , # [doc = " The reason the QUIC connection was closed"] connection_close_reason : RwLock < Option < BoxError > > , # [doc = " Max recorded bandwidth."] max_bandwidth : AtomicU64 , # [doc = " Loss at max recorded bandwidth."] max_loss_pct : AtomicU8 , # [doc = " The value of the first `SO_RECVMARK` control message received for the"] # [doc = " connection."] # [doc = ""] # [doc = " Linux-only."] # [cfg (target_os = "linux")] initial_so_mark : OnceLock < [u8 ; 4] > , # [doc = " The server's chosen QUIC connection ID."] # [doc = ""] # [doc = " The QUIC connection ID is presently an array of 20 bytes (160 bits)"] pub quic_connection_id : Vec < u8 > , }
};
}
