// Generated macro for Error (enum)
macro_rules! Depcrate_encodeError {
() => {
// Module: crate::encode
// Provides: {"Error"}
// Dependencies: {}
# [doc = " This type represents all possible errors that can occur when serializing or"] # [doc = " deserializing MessagePack data."] # [derive (Debug)] pub enum Error { # [doc = " Failed to write a MessagePack value."] InvalidValueWrite (ValueWriteError) , # [doc = " Failed to serialize struct, sequence or map, because its length is unknown."] UnknownLength , # [doc = " Invalid Data model, i.e. Serialize trait is not implmented correctly"] InvalidDataModel (& 'static str) , # [doc = " Depth limit exceeded"] DepthLimitExceeded , # [doc = " Catchall for syntax error messages."] Syntax (String) , }
};
}
