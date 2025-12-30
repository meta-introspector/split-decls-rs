// Generated macro for QpackHeadersDecoded (struct)
macro_rules! Depcrate_events_qpackQpackHeadersDecoded {
() => {
// Module: crate::events::qpack
// Provides: {"QpackHeadersDecoded"}
// Dependencies: {}
# [serde_with :: skip_serializing_none] # [derive (Serialize , Deserialize , Clone , PartialEq , Eq , Debug)] pub struct QpackHeadersDecoded { pub stream_id : Option < u64 > , pub headers : Option < HttpHeader > , pub block_prefix : QpackHeaderBlockPrefix , pub header_block : Vec < QpackHeaderBlockRepresentation > , pub raw : Option < RawInfo > , }
};
}
