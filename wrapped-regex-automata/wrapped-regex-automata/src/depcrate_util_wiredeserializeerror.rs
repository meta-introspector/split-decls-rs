// Generated macro for DeserializeError (struct)
macro_rules! Depcrate_util_wireDeserializeError {
() => {
// Module: crate::util::wire
// Provides: {"DeserializeError"}
// Dependencies: {}
# [doc = " An error that occurs when deserializing an object defined in this crate."] # [doc = ""] # [doc = " Serialization, as used in this crate, universally refers to the process"] # [doc = " of transforming a structure (like a DFA) into a custom binary format"] # [doc = " represented by `&[u8]`. Deserialization, then, refers to the process of"] # [doc = " cheaply converting this binary format back to the object's in-memory"] # [doc = " representation as defined in this crate. To the extent possible,"] # [doc = " deserialization will report this error whenever this process fails."] # [doc = ""] # [doc = " A `DeserializeError` provides no introspection capabilities. Its only"] # [doc = " supported operation is conversion to a human readable error message."] # [doc = ""] # [doc = " This error type implements the `std::error::Error` trait only when the"] # [doc = " `std` feature is enabled. Otherwise, this type is defined in all"] # [doc = " configurations."] # [derive (Debug)] pub struct DeserializeError (DeserializeErrorKind) ;
};
}
