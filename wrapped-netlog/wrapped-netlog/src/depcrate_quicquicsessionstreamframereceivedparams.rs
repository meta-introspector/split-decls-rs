// Generated macro for QuicSessionStreamFrameReceivedParams (struct)
macro_rules! Depcrate_quicQuicSessionStreamFrameReceivedParams {
() => {
// Module: crate::quic
// Provides: {"QuicSessionStreamFrameReceivedParams"}
// Dependencies: {}
# [derive (Serialize , Deserialize , Debug , Default)] pub struct QuicSessionStreamFrameReceivedParams { pub stream_id : u64 , pub fin : bool , pub offset : u64 , pub length : u64 , }
};
}
