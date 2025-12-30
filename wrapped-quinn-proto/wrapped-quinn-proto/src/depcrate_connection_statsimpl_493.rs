// Generated macro for impl_493 (impl)
macro_rules! Depcrate_connection_statsimpl_493 {
() => {
// Module: crate::connection::stats
// Provides: {"impl_493"}
// Dependencies: {}
impl FrameStats { pub (crate) fn record (& mut self , frame : & Frame) { match frame { Frame :: Padding => { } Frame :: Ping => self . ping += 1 , Frame :: Ack (_) => self . acks += 1 , Frame :: ResetStream (_) => self . reset_stream += 1 , Frame :: StopSending (_) => self . stop_sending += 1 , Frame :: Crypto (_) => self . crypto += 1 , Frame :: Datagram (_) => self . datagram += 1 , Frame :: NewToken (_) => self . new_token += 1 , Frame :: MaxData (_) => self . max_data += 1 , Frame :: MaxStreamData { .. } => self . max_stream_data += 1 , Frame :: MaxStreams { dir , .. } => { if * dir == Dir :: Bi { self . max_streams_bidi += 1 ; } else { self . max_streams_uni += 1 ; } } Frame :: DataBlocked { .. } => self . data_blocked += 1 , Frame :: Stream (_) => self . stream += 1 , Frame :: StreamDataBlocked { .. } => self . stream_data_blocked += 1 , Frame :: StreamsBlocked { dir , .. } => { if * dir == Dir :: Bi { self . streams_blocked_bidi += 1 ; } else { self . streams_blocked_uni += 1 ; } } Frame :: NewConnectionId (_) => self . new_connection_id += 1 , Frame :: RetireConnectionId { .. } => self . retire_connection_id += 1 , Frame :: PathChallenge (_) => self . path_challenge += 1 , Frame :: PathResponse (_) => self . path_response += 1 , Frame :: Close (_) => self . connection_close += 1 , Frame :: AckFrequency (_) => self . ack_frequency += 1 , Frame :: ImmediateAck => self . immediate_ack += 1 , Frame :: HandshakeDone => self . handshake_done = self . handshake_done . saturating_add (1) , } } }
};
}
