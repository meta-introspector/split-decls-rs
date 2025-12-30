// Generated macro for try_os_str_into_bstr (function)
macro_rules! Depcrate_converttry_os_str_into_bstr {
() => {
// Module: crate::convert
// Provides: {"try_os_str_into_bstr"}
// Dependencies: {}
# [doc = " Like [`into_bstr()`], but takes `Cow<OsStr>` as input for a lossless, but fallible, conversion."] pub fn try_os_str_into_bstr (path : Cow < '_ , OsStr >) -> Result < Cow < '_ , BStr > , Utf8Error > { match path { Cow :: Borrowed (path) => os_str_into_bstr (path) . map (Cow :: Borrowed) , Cow :: Owned (path) => os_string_into_bstring (path) . map (Cow :: Owned) , } }
};
}
