// Generated macro for guess_mime_type (function)
macro_rules! Depcrateguess_mime_type {
() => {
// Module: crate
// Provides: {"guess_mime_type"}
// Dependencies: {}
# [doc = " Guess the MIME type of `path` by its extension (as defined by `Path::extension()`)."] # [doc = ""] # [doc = " If `path` has no extension, or its extension has no known MIME type mapping,"] # [doc = " then the MIME type is assumed to be `application/octet-stream`."] # [doc = ""] # [doc = " ## Note"] # [doc = " **Guess** is the operative word here, as there are no guarantees that the contents of the file"] # [doc = " that `path` points to match the MIME type associated with the path's extension."] # [doc = ""] # [doc = " Take care when processing files with assumptions based on the return value of this function."] # [doc = ""] # [doc = " In HTTP applications, it might be [preferable][rfc7231] to not send a `Content-Type`"] # [doc = " header at all instead of defaulting to `application/octet-stream`."] # [doc = ""] # [doc = " [rfc7231]: https://tools.ietf.org/html/rfc7231#section-3.1.1.5"] # [deprecated (since = "2.0.0" , note = "Use `from_path(path).first_or_octet_stream()` instead")] pub fn guess_mime_type < P : AsRef < Path > > (path : P) -> Mime { from_path (path) . first_or_octet_stream () }
};
}
