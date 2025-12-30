// Generated macro for fake_packet_sent (function)
macro_rules! Depcratefake_packet_sent {
() => {
// Module: crate
// Provides: {"fake_packet_sent"}
// Dependencies: {}
fn fake_packet_sent (frames : Option < SmallVec < [QuicFrame ; 1] > >) -> EventData { EventData :: PacketSent (PacketSent { header : fake_packet_header () , frames , .. Default :: default () }) }
};
}
