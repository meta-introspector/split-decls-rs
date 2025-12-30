// Generated macro for guess_mime_type_opt (function)
macro_rules! Depcrateguess_mime_type_opt {
() => {
// Module: crate
// Provides: {"guess_mime_type_opt"}
// Dependencies: {}
# [doc = " Guess the MIME type of `path` by its extension (as defined by `Path::extension()`)."] # [doc = ""] # [doc = " If `path` has no extension, or its extension has no known MIME type mapping,"] # [doc = " then `None` is returned."] # [doc = ""] # [deprecated (since = "2.0.0" , note = "Use `from_path(path).first()` instead")] pub fn guess_mime_type_opt < P : AsRef < Path > > (path : P) -> Option < Mime > { from_path (path) . first () }
};
}
