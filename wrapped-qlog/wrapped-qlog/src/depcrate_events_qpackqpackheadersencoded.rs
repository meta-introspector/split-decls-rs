// Generated macro for QpackHeadersEncoded (struct)
macro_rules! Depcrate_events_qpackQpackHeadersEncoded {
() => {
// Module: crate::events::qpack
// Provides: {"QpackHeadersEncoded"}
// Dependencies: {}
# [serde_with :: skip_serializing_none] # [derive (Serialize , Deserialize , Clone , PartialEq , Eq , Debug)] pub struct QpackHeadersEncoded { pub stream_id : Option < u64 > , pub headers : Option < HttpHeader > , pub block_prefix : QpackHeaderBlockPrefix , pub header_block : Vec < QpackHeaderBlockRepresentation > , pub raw : Option < RawInfo > , }
};
}
