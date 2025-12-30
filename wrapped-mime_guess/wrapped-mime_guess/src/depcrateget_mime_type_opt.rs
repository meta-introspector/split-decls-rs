// Generated macro for get_mime_type_opt (function)
macro_rules! Depcrateget_mime_type_opt {
() => {
// Module: crate
// Provides: {"get_mime_type_opt"}
// Dependencies: {}
# [doc = " Get the MIME type associated with a file extension."] # [doc = ""] # [doc = " If there is no association for the extension, or `ext` is empty,"] # [doc = " `None` is returned."] # [deprecated (since = "2.0.0" , note = "use `from_ext(search_ext).first()` instead")] pub fn get_mime_type_opt (search_ext : & str) -> Option < Mime > { from_ext (search_ext) . first () }
};
}
