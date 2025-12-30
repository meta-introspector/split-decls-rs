// Generated macro for ValueReadError (enum)
macro_rules! Depcrate_decodeValueReadError {
() => {
// Module: crate::decode
// Provides: {"ValueReadError"}
// Dependencies: {}
# [doc = " An error which can occur when attempting to read a MessagePack value from the reader."] # [derive (Debug)] # [allow (deprecated)] pub enum ValueReadError < E : RmpReadErr = Error > { # [doc = " Failed to read the marker."] InvalidMarkerRead (E) , # [doc = " Failed to read the data."] InvalidDataRead (E) , # [doc = " The type decoded isn't match with the expected one."] TypeMismatch (Marker) , }
};
}
