// Generated macro for DataMoved (struct)
macro_rules! Depcrate_events_quicDataMoved {
() => {
// Module: crate::events::quic
// Provides: {"DataMoved"}
// Dependencies: {}
# [serde_with :: skip_serializing_none] # [derive (Serialize , Deserialize , Clone , PartialEq , Eq , Debug , Default)] pub struct DataMoved { pub stream_id : Option < u64 > , pub offset : Option < u64 > , pub length : Option < u64 > , pub from : Option < DataRecipient > , pub to : Option < DataRecipient > , pub raw : Option < RawInfo > , }
};
}
