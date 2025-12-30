// Generated macro for get_mime_type (function)
macro_rules! Depcrateget_mime_type {
() => {
// Module: crate
// Provides: {"get_mime_type"}
// Dependencies: {}
# [doc = " Get the MIME type associated with a file extension."] # [doc = ""] # [doc = " If there is no association for the extension, or `ext` is empty,"] # [doc = " `application/octet-stream` is returned."] # [doc = ""] # [doc = " ## Note"] # [doc = " In HTTP applications, it might be [preferable][rfc7231] to not send a `Content-Type`"] # [doc = " header at all instead of defaulting to `application/octet-stream`."] # [doc = ""] # [doc = " [rfc7231]: https://tools.ietf.org/html/rfc7231#section-3.1.1.5"] # [deprecated (since = "2.0.0" , note = "use `from_ext(search_ext).first_or_octet_stream()` instead")] pub fn get_mime_type (search_ext : & str) -> Mime { from_ext (search_ext) . first_or_octet_stream () }
};
}
