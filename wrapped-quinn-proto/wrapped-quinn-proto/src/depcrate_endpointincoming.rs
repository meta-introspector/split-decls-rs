// Generated macro for Incoming (struct)
macro_rules! Depcrate_endpointIncoming {
() => {
// Module: crate::endpoint
// Provides: {"Incoming"}
// Dependencies: {}
# [doc = " An incoming connection for which the server has not yet begun its part of the handshake."] pub struct Incoming { received_at : Instant , addresses : FourTuple , ecn : Option < EcnCodepoint > , packet : InitialPacket , rest : Option < BytesMut > , crypto : Keys , token : IncomingToken , incoming_idx : usize , improper_drop_warner : IncomingImproperDropWarner , }
};
}
