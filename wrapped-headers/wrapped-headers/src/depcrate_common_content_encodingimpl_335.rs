// Generated macro for impl_335 (impl)
macro_rules! Depcrate_common_content_encodingimpl_335 {
() => {
// Module: crate::common::content_encoding
// Provides: {"impl_335"}
// Dependencies: {}
impl ContentEncoding { # [doc = " A constructor to easily create a `Content-Encoding: gzip` header."] # [inline] pub fn gzip () -> ContentEncoding { ContentEncoding (HeaderValue :: from_static ("gzip") . into ()) } # [doc = " A constructor to easily create a `Content-Encoding: br` header."] # [inline] pub fn brotli () -> ContentEncoding { ContentEncoding (HeaderValue :: from_static ("br") . into ()) } # [doc = " A constructor to easily create a `Content-Encoding: zstd` header."] # [inline] pub fn zstd () -> ContentEncoding { ContentEncoding (HeaderValue :: from_static ("zstd") . into ()) } # [doc = " Check if this header contains a given \"coding\"."] # [doc = ""] # [doc = " This can be used with these argument types:"] # [doc = ""] # [doc = " - `&str`"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use headers::ContentEncoding;"] # [doc = ""] # [doc = " let content_enc = ContentEncoding::gzip();"] # [doc = ""] # [doc = " assert!(content_enc.contains(\"gzip\"));"] # [doc = " assert!(!content_enc.contains(\"br\"));"] # [doc = " ```"] pub fn contains (& self , coding : impl AsCoding) -> bool { let s = coding . as_coding () ; self . 0 . iter () . any (| opt | opt == s) } }
};
}
