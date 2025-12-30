// Generated macro for os_str_into_bstr (function)
macro_rules! Depcrate_convertos_str_into_bstr {
() => {
// Module: crate::convert
// Provides: {"os_str_into_bstr"}
// Dependencies: {}
# [doc = " Like [`into_bstr()`], but takes `OsStr` as input for a lossless, but fallible, conversion."] pub fn os_str_into_bstr (path : & OsStr) -> Result < & BStr , Utf8Error > { let path = try_into_bstr (Cow :: Borrowed (path . as_ref ())) ? ; match path { Cow :: Borrowed (path) => Ok (path) , Cow :: Owned (_) => unreachable ! ("borrowed cows stay borrowed") , } }
};
}
