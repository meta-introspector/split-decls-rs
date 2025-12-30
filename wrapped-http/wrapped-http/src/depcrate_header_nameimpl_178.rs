// Generated macro for impl_178 (impl)
macro_rules! Depcrate_header_nameimpl_178 {
() => {
// Module: crate::header::name
// Provides: {"impl_178"}
// Dependencies: {}
impl PartialEq < str > for HeaderName { # [doc = " Performs a case-insensitive comparison of the string against the header"] # [doc = " name"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use http::header::CONTENT_LENGTH;"] # [doc = ""] # [doc = " assert_eq!(CONTENT_LENGTH, \"content-length\");"] # [doc = " assert_eq!(CONTENT_LENGTH, \"Content-Length\");"] # [doc = " assert_ne!(CONTENT_LENGTH, \"content length\");"] # [doc = " ```"] # [inline] fn eq (& self , other : & str) -> bool { eq_ignore_ascii_case (self . as_ref () , other . as_bytes ()) } }
};
}
