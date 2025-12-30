// Generated macro for DecodeOwned (trait)
macro_rules! Depcrate_decodeDecodeOwned {
() => {
// Module: crate::decode
// Provides: {"DecodeOwned"}
// Dependencies: {}
# [doc = " Marker trait for data structures that can be decoded from DER without"] # [doc = " borrowing any data from the decoder."] # [doc = ""] # [doc = " This is primarily useful for trait bounds on functions which require that"] # [doc = " no data is borrowed from the decoder, for example a PEM decoder which needs"] # [doc = " to first decode data from Base64."] # [doc = ""] # [doc = " This trait is inspired by the [`DeserializeOwned` trait from `serde`](https://docs.rs/serde/latest/serde/de/trait.DeserializeOwned.html)."] # [diagnostic :: on_unimplemented (note = "`DecodeOwned` is auto-impl'd for all lifetime-free types which impl `Decode`")] pub trait DecodeOwned : for < 'a > Decode < 'a > { }
};
}
