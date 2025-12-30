// Generated macro for Tagged (struct)
macro_rules! Depcrate_tagsTagged {
() => {
// Module: crate::tags
// Provides: {"Tagged"}
// Dependencies: {}
# [doc = " A value that is optionally tagged with a cbor tag"] # [doc = ""] # [doc = " this only serves as an intermediate helper for tag serialization or deserialization"] pub struct Tagged < T > { # [doc = " cbor tag"] pub tag : Option < u64 > , # [doc = " value"] pub value : T , }
};
}
