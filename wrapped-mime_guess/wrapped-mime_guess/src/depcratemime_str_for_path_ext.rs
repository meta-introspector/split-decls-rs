// Generated macro for mime_str_for_path_ext (function)
macro_rules! Depcratemime_str_for_path_ext {
() => {
// Module: crate
// Provides: {"mime_str_for_path_ext"}
// Dependencies: {}
# [doc = " Guess the MIME type string of `path` by its extension (as defined by `Path::extension()`)."] # [doc = ""] # [doc = " If `path` has no extension, or its extension has no known MIME type mapping,"] # [doc = " then `None` is returned."] # [doc = ""] # [doc = " ## Note"] # [doc = " **Guess** is the operative word here, as there are no guarantees that the contents of the file"] # [doc = " that `path` points to match the MIME type associated with the path's extension."] # [doc = ""] # [doc = " Take care when processing files with assumptions based on the return value of this function."] # [deprecated (since = "2.0.0" , note = "Use `from_path(path).first_raw()` instead")] pub fn mime_str_for_path_ext < P : AsRef < Path > > (path : P) -> Option < & 'static str > { from_path (path) . first_raw () }
};
}
