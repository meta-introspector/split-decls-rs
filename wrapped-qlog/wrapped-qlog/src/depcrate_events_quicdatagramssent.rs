// Generated macro for DatagramsSent (struct)
macro_rules! Depcrate_events_quicDatagramsSent {
() => {
// Module: crate::events::quic
// Provides: {"DatagramsSent"}
// Dependencies: {}
# [serde_with :: skip_serializing_none] # [derive (Serialize , Deserialize , Clone , PartialEq , Eq , Debug , Default)] pub struct DatagramsSent { pub count : Option < u16 > , pub raw : Option < Vec < RawInfo > > , pub datagram_ids : Option < Vec < u32 > > , }
};
}
