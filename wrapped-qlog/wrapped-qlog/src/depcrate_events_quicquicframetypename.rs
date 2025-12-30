// Generated macro for QuicFrameTypeName (enum)
macro_rules! Depcrate_events_quicQuicFrameTypeName {
() => {
// Module: crate::events::quic
// Provides: {"QuicFrameTypeName"}
// Dependencies: {}
# [derive (Serialize , Deserialize , Clone , PartialEq , Eq , Debug , Default)] # [serde (rename_all = "snake_case")] pub enum QuicFrameTypeName { Padding , Ping , Ack , ResetStream , StopSending , Crypto , NewToken , Stream , MaxData , MaxStreamData , MaxStreams , DataBlocked , StreamDataBlocked , StreamsBlocked , NewConnectionId , RetireConnectionId , PathChallenge , PathResponse , ConnectionClose , ApplicationClose , HandshakeDone , Datagram , # [default] Unknown , }
};
}
