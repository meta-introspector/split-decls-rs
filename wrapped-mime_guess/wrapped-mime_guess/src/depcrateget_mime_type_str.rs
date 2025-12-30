// Generated macro for get_mime_type_str (function)
macro_rules! Depcrateget_mime_type_str {
() => {
// Module: crate
// Provides: {"get_mime_type_str"}
// Dependencies: {}
# [doc = " Get the MIME type string associated with a file extension. Case-insensitive."] # [doc = ""] # [doc = " If `search_ext` is not already lowercase,"] # [doc = " it will be converted to lowercase to facilitate the search."] # [doc = ""] # [doc = " Returns `None` if `search_ext` is empty or an associated extension was not found."] # [deprecated (since = "2.0.0" , note = "use `from_ext(search_ext).first_raw()` instead")] pub fn get_mime_type_str (search_ext : & str) -> Option < & 'static str > { from_ext (search_ext) . first_raw () }
};
}
