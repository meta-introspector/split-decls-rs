// Generated macro for QuicSessionRstStreamFrameReceivedParams (struct)
macro_rules! Depcrate_quicQuicSessionRstStreamFrameReceivedParams {
() => {
// Module: crate::quic
// Provides: {"QuicSessionRstStreamFrameReceivedParams"}
// Dependencies: {}
# [derive (Serialize , Deserialize , Debug , Default)] pub struct QuicSessionRstStreamFrameReceivedParams { pub stream_id : u64 , pub quic_rst_stream_error : u64 , pub offset : u64 , }
};
}
