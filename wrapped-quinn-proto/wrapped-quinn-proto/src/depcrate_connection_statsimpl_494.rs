// Generated macro for impl_494 (impl)
macro_rules! Depcrate_connection_statsimpl_494 {
() => {
// Module: crate::connection::stats
// Provides: {"impl_494"}
// Dependencies: {}
impl std :: fmt :: Debug for FrameStats { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . debug_struct ("FrameStats") . field ("ACK" , & self . acks) . field ("ACK_FREQUENCY" , & self . ack_frequency) . field ("CONNECTION_CLOSE" , & self . connection_close) . field ("CRYPTO" , & self . crypto) . field ("DATA_BLOCKED" , & self . data_blocked) . field ("DATAGRAM" , & self . datagram) . field ("HANDSHAKE_DONE" , & self . handshake_done) . field ("IMMEDIATE_ACK" , & self . immediate_ack) . field ("MAX_DATA" , & self . max_data) . field ("MAX_STREAM_DATA" , & self . max_stream_data) . field ("MAX_STREAMS_BIDI" , & self . max_streams_bidi) . field ("MAX_STREAMS_UNI" , & self . max_streams_uni) . field ("NEW_CONNECTION_ID" , & self . new_connection_id) . field ("NEW_TOKEN" , & self . new_token) . field ("PATH_CHALLENGE" , & self . path_challenge) . field ("PATH_RESPONSE" , & self . path_response) . field ("PING" , & self . ping) . field ("RESET_STREAM" , & self . reset_stream) . field ("RETIRE_CONNECTION_ID" , & self . retire_connection_id) . field ("STREAM_DATA_BLOCKED" , & self . stream_data_blocked) . field ("STREAMS_BLOCKED_BIDI" , & self . streams_blocked_bidi) . field ("STREAMS_BLOCKED_UNI" , & self . streams_blocked_uni) . field ("STOP_SENDING" , & self . stop_sending) . field ("STREAM" , & self . stream) . finish () } }
};
}
