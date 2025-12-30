// Generated macro for apply_params (macro)
macro_rules! Depcrate_transport_parametersapply_params {
() => {
// Module: crate::transport_parameters
// Provides: {"apply_params"}
// Dependencies: {}
macro_rules ! apply_params { ($ macro : ident) => { $ macro ! { # [doc = " Milliseconds, disabled if zero"] max_idle_timeout (MaxIdleTimeout) = 0 , # [doc = " Limits the size of UDP payloads that the endpoint is willing to receive"] max_udp_payload_size (MaxUdpPayloadSize) = 65527 , # [doc = " Initial value for the maximum amount of data that can be sent on the connection"] initial_max_data (InitialMaxData) = 0 , # [doc = " Initial flow control limit for locally-initiated bidirectional streams"] initial_max_stream_data_bidi_local (InitialMaxStreamDataBidiLocal) = 0 , # [doc = " Initial flow control limit for peer-initiated bidirectional streams"] initial_max_stream_data_bidi_remote (InitialMaxStreamDataBidiRemote) = 0 , # [doc = " Initial flow control limit for unidirectional streams"] initial_max_stream_data_uni (InitialMaxStreamDataUni) = 0 , # [doc = " Initial maximum number of bidirectional streams the peer may initiate"] initial_max_streams_bidi (InitialMaxStreamsBidi) = 0 , # [doc = " Initial maximum number of unidirectional streams the peer may initiate"] initial_max_streams_uni (InitialMaxStreamsUni) = 0 , # [doc = " Exponent used to decode the ACK Delay field in the ACK frame"] ack_delay_exponent (AckDelayExponent) = 3 , # [doc = " Maximum amount of time in milliseconds by which the endpoint will delay sending"] # [doc = " acknowledgments"] max_ack_delay (MaxAckDelay) = 25 , # [doc = " Maximum number of connection IDs from the peer that an endpoint is willing to store"] active_connection_id_limit (ActiveConnectionIdLimit) = 2 , } } ; }
};
}
