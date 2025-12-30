// Generated macro for ContentEncoding (struct)
macro_rules! Depcrate_common_content_encodingContentEncoding {
() => {
// Module: crate::common::content_encoding
// Provides: {"ContentEncoding"}
// Dependencies: {}
# [doc = " `Content-Encoding` header, defined in"] # [doc = " [RFC7231](https://datatracker.ietf.org/doc/html/rfc7231#section-3.1.2.2)"] # [doc = ""] # [doc = " The `Content-Encoding` header field indicates what content codings"] # [doc = " have been applied to the representation, beyond those inherent in the"] # [doc = " media type, and thus what decoding mechanisms have to be applied in"] # [doc = " order to obtain data in the media type referenced by the Content-Type"] # [doc = " header field.  Content-Encoding is primarily used to allow a"] # [doc = " representation's data to be compressed without losing the identity of"] # [doc = " its underlying media type."] # [doc = ""] # [doc = " # ABNF"] # [doc = ""] # [doc = " ```text"] # [doc = " Content-Encoding = 1#content-coding"] # [doc = " ```"] # [doc = ""] # [doc = " # Example values"] # [doc = ""] # [doc = " * `gzip`"] # [doc = " * `br`"] # [doc = " * `zstd`"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use headers::ContentEncoding;"] # [doc = ""] # [doc = " let content_enc = ContentEncoding::gzip();"] # [doc = " ```"] # [derive (Clone , Debug)] pub struct ContentEncoding (FlatCsv) ;
};
}
