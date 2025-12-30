// Generated macro for impl_363 (impl)
macro_rules! Depcrate_common_content_typeimpl_363 {
() => {
// Module: crate::common::content_type
// Provides: {"impl_363"}
// Dependencies: {}
impl ContentType { # [doc = " A constructor  to easily create a `Content-Type: application/json` header."] # [inline] pub fn json () -> ContentType { ContentType (mime :: APPLICATION_JSON) } # [doc = " A constructor  to easily create a `Content-Type: text/plain` header."] # [inline] pub fn text () -> ContentType { ContentType (mime :: TEXT_PLAIN) } # [doc = " A constructor  to easily create a `Content-Type: text/plain; charset=utf-8` header."] # [inline] pub fn text_utf8 () -> ContentType { ContentType (mime :: TEXT_PLAIN_UTF_8) } # [doc = " A constructor  to easily create a `Content-Type: text/html` header."] # [inline] pub fn html () -> ContentType { ContentType (mime :: TEXT_HTML) } # [doc = " A constructor  to easily create a `Content-Type: text/xml` header."] # [inline] pub fn xml () -> ContentType { ContentType (mime :: TEXT_XML) } # [doc = " A constructor  to easily create a `Content-Type: application/www-form-url-encoded` header."] # [inline] pub fn form_url_encoded () -> ContentType { ContentType (mime :: APPLICATION_WWW_FORM_URLENCODED) } # [doc = " A constructor  to easily create a `Content-Type: image/jpeg` header."] # [inline] pub fn jpeg () -> ContentType { ContentType (mime :: IMAGE_JPEG) } # [doc = " A constructor  to easily create a `Content-Type: image/png` header."] # [inline] pub fn png () -> ContentType { ContentType (mime :: IMAGE_PNG) } # [doc = " A constructor  to easily create a `Content-Type: application/octet-stream` header."] # [inline] pub fn octet_stream () -> ContentType { ContentType (mime :: APPLICATION_OCTET_STREAM) } }
};
}
