// Generated macro for ByteBuf (struct)
macro_rules! Depcrate_utilsByteBuf {
() => {
// Module: crate::utils
// Provides: {"ByteBuf"}
// Dependencies: {}
# [doc = " Wrapper around `Vec<u8>` that has a human-readable debug representation:"] # [doc = " printable ASCII symbols output as is, all other output in HEX notation."] # [doc = ""] # [doc = " Also, when [`serialize`] feature is on, this type deserialized using"] # [doc = " [`deserialize_byte_buf`](serde::Deserializer::deserialize_byte_buf) instead"] # [doc = " of vector's generic [`deserialize_seq`](serde::Deserializer::deserialize_seq)"] # [doc = ""] # [doc = " [`serialize`]: ../index.html#serialize"] # [derive (PartialEq , Eq)] pub struct ByteBuf (pub Vec < u8 >) ;
};
}
