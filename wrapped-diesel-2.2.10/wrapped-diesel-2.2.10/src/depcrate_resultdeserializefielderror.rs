// Generated macro for DeserializeFieldError (struct)
macro_rules! Depcrate_resultDeserializeFieldError {
() => {
// Module: crate::result
// Provides: {"DeserializeFieldError"}
// Dependencies: {}
# [doc = " An error occurred while deserializing a field"] # [derive (Debug)] # [non_exhaustive] pub struct DeserializeFieldError { # [doc = " The name of the field that failed to deserialize"] pub field_name : Option < String > , # [doc = " The error that occurred while deserializing the field"] pub error : Box < dyn StdError + Send + Sync > , }
};
}
