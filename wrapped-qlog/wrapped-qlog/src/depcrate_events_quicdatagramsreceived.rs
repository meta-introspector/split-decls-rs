// Generated macro for DatagramsReceived (struct)
macro_rules! Depcrate_events_quicDatagramsReceived {
() => {
// Module: crate::events::quic
// Provides: {"DatagramsReceived"}
// Dependencies: {}
# [serde_with :: skip_serializing_none] # [derive (Serialize , Deserialize , Clone , PartialEq , Eq , Debug , Default)] pub struct DatagramsReceived { pub count : Option < u16 > , pub raw : Option < Vec < RawInfo > > , pub datagram_ids : Option < Vec < u32 > > , }
};
}
