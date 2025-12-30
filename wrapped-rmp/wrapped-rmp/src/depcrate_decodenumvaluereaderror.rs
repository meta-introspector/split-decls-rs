// Generated macro for NumValueReadError (enum)
macro_rules! Depcrate_decodeNumValueReadError {
() => {
// Module: crate::decode
// Provides: {"NumValueReadError"}
// Dependencies: {}
# [doc = " An error which can occur when attempting to read a MessagePack numeric value from the reader."] # [derive (Debug)] # [allow (deprecated)] pub enum NumValueReadError < E : RmpReadErr = Error > { # [doc = " Failed to read the marker."] InvalidMarkerRead (E) , # [doc = " Failed to read the data."] InvalidDataRead (E) , # [doc = " The type decoded isn't match with the expected one."] TypeMismatch (Marker) , # [doc = " Out of range integral type conversion attempted."] OutOfRange , }
};
}
