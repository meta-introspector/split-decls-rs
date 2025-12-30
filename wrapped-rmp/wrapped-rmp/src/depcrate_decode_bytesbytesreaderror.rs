// Generated macro for BytesReadError (enum)
macro_rules! Depcrate_decode_bytesBytesReadError {
() => {
// Module: crate::decode::bytes
// Provides: {"BytesReadError"}
// Dependencies: {}
# [doc = " Indicates that an error occurred reading from [Bytes]"] # [derive (Debug)] # [non_exhaustive] pub enum BytesReadError { # [doc = " Indicates that there were not enough bytes."] InsufficientBytes { expected : usize , actual : usize , position : u64 , } , }
};
}
