// Generated macro for ValueWriteError (enum)
macro_rules! Depcrate_encodeValueWriteError {
() => {
// Module: crate::encode
// Provides: {"ValueWriteError"}
// Dependencies: {}
# [doc = " An error that can occur when attempting to write multi-byte MessagePack value."] # [derive (Debug)] # [allow (deprecated)] pub enum ValueWriteError < E : RmpWriteErr = Error > { # [doc = " I/O error while writing marker."] InvalidMarkerWrite (E) , # [doc = " I/O error while writing data."] InvalidDataWrite (E) , }
};
}
