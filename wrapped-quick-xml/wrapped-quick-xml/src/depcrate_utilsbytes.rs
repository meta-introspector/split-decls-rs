// Generated macro for Bytes (struct)
macro_rules! Depcrate_utilsBytes {
() => {
// Module: crate::utils
// Provides: {"Bytes"}
// Dependencies: {}
# [doc = " Wrapper around `&[u8]` that has a human-readable debug representation:"] # [doc = " printable ASCII symbols output as is, all other output in HEX notation."] # [doc = ""] # [doc = " Also, when [`serialize`] feature is on, this type deserialized using"] # [doc = " [`deserialize_bytes`](serde::Deserializer::deserialize_bytes) instead"] # [doc = " of vector's generic [`deserialize_seq`](serde::Deserializer::deserialize_seq)"] # [doc = ""] # [doc = " [`serialize`]: ../index.html#serialize"] # [derive (PartialEq , Eq)] pub struct Bytes < 'de > (pub & 'de [u8]) ;
};
}
