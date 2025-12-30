// Generated macro for Frame (enum)
macro_rules! Depcrate_frameFrame {
() => {
// Module: crate::frame
// Provides: {"Frame"}
// Dependencies: {}
# [derive (Debug)] pub (crate) enum Frame { Padding , Ping , Ack (Ack) , ResetStream (ResetStream) , StopSending (StopSending) , Crypto (Crypto) , NewToken (NewToken) , Stream (Stream) , MaxData (VarInt) , MaxStreamData { id : StreamId , offset : u64 } , MaxStreams { dir : Dir , count : u64 } , DataBlocked { offset : u64 } , StreamDataBlocked { id : StreamId , offset : u64 } , StreamsBlocked { dir : Dir , limit : u64 } , NewConnectionId (NewConnectionId) , RetireConnectionId { sequence : u64 } , PathChallenge (u64) , PathResponse (u64) , Close (Close) , Datagram (Datagram) , AckFrequency (AckFrequency) , ImmediateAck , HandshakeDone , }
};
}
