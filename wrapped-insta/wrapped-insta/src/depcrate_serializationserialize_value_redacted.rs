// Generated macro for serialize_value_redacted (function)
macro_rules! Depcrate_serializationserialize_value_redacted {
() => {
// Module: crate::serialization
// Provides: {"serialize_value_redacted"}
// Dependencies: {}
# [cfg (feature = "redactions")] pub fn serialize_value_redacted < S : Serialize > (s : & S , redactions : & [(crate :: redaction :: Selector , crate :: redaction :: Redaction)] , format : SerializationFormat ,) -> String { let serializer = ContentSerializer :: < ValueError > :: new () ; let mut content = Serialize :: serialize (s , serializer) . unwrap () ; for (selector , redaction) in redactions { content = selector . redact (content , redaction) ; } serialize_content (content , format) }
};
}
