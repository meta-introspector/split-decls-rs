// Generated macro for DecodeStringError (enum)
macro_rules! Depcrate_decode_strDecodeStringError {
() => {
// Module: crate::decode::str
// Provides: {"DecodeStringError"}
// Dependencies: {}
# [derive (Debug)] # [allow (deprecated)] pub enum DecodeStringError < 'a , E : RmpReadErr = super :: Error > { InvalidMarkerRead (E) , InvalidDataRead (E) , TypeMismatch (Marker) , # [doc = " The given buffer is not large enough to accumulate the specified amount of bytes."] BufferSizeTooSmall (u32) , InvalidUtf8 (& 'a [u8] , Utf8Error) , }
};
}
