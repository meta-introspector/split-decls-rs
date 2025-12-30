// Generated macro for Decoder (struct)
macro_rules! Depcrate_encodingDecoder {
() => {
// Module: crate::encoding
// Provides: {"Decoder"}
// Dependencies: {}
# [doc = " Decoder of byte slices into strings."] # [doc = ""] # [doc = " If feature [`encoding`] is enabled, this encoding taken from the `\"encoding\"`"] # [doc = " XML declaration or assumes UTF-8, if XML has no <?xml ?> declaration, encoding"] # [doc = " key is not defined or contains unknown encoding."] # [doc = ""] # [doc = " The library supports any UTF-8 compatible encodings that crate `encoding_rs`"] # [doc = " is supported. [*UTF-16 and ISO-2022-JP are not supported at the present*][utf16]."] # [doc = ""] # [doc = " If feature [`encoding`] is disabled, the decoder is always UTF-8 decoder:"] # [doc = " any XML declarations are ignored."] # [doc = ""] # [doc = " [utf16]: https://github.com/tafia/quick-xml/issues/158"] # [doc = " [`encoding`]: ../index.html#encoding"] # [derive (Clone , Copy , Debug , Eq , PartialEq)] pub struct Decoder { # [cfg (feature = "encoding")] pub (crate) encoding : & 'static Encoding , }
};
}
