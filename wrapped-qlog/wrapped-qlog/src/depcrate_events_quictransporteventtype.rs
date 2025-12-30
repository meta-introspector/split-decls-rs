// Generated macro for TransportEventType (enum)
macro_rules! Depcrate_events_quicTransportEventType {
() => {
// Module: crate::events::quic
// Provides: {"TransportEventType"}
// Dependencies: {}
# [derive (Serialize , Deserialize , Clone , Copy , PartialEq , Eq , Debug)] # [serde (rename_all = "snake_case")] pub enum TransportEventType { VersionInformation , AlpnInformation , ParametersSet , ParametersRestored , DatagramsSent , DatagramsReceived , DatagramDropped , PacketSent , PacketReceived , PacketDropped , PacketBuffered , PacketsAcked , FramesProcessed , StreamStateUpdated , DataMoved , }
};
}
