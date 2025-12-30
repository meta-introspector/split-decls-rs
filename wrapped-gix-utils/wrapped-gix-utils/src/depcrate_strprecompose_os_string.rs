// Generated macro for precompose_os_string (function)
macro_rules! Depcrate_strprecompose_os_string {
() => {
// Module: crate::str
// Provides: {"precompose_os_string"}
// Dependencies: {}
# [doc = " Return the precomposed version of `name`, or `name` itself if it contained illformed unicode,"] # [doc = " or if the unicode version didn't contains decomposed unicode."] # [doc = " Otherwise, similar to [`precompose()`]"] pub fn precompose_os_string (name : Cow < '_ , OsStr >) -> Cow < '_ , OsStr > { match name . to_str () { None => name , Some (maybe_decomposed) => match precompose (maybe_decomposed . into ()) { Cow :: Borrowed (_) => name , Cow :: Owned (precomposed) => Cow :: Owned (precomposed . into ()) , } , } }
};
}
