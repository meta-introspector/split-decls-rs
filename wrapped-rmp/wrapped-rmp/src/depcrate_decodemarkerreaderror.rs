// Generated macro for MarkerReadError (struct)
macro_rules! Depcrate_decodeMarkerReadError {
() => {
// Module: crate::decode
// Provides: {"MarkerReadError"}
// Dependencies: {}
# [doc = " An error that can occur when attempting to read a MessagePack marker from the reader."] # [derive (Debug)] # [allow (deprecated)] pub struct MarkerReadError < E : RmpReadErr = Error > (pub E) ;
};
}
