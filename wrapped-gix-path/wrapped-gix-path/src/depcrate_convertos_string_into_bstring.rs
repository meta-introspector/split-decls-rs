// Generated macro for os_string_into_bstring (function)
macro_rules! Depcrate_convertos_string_into_bstring {
() => {
// Module: crate::convert
// Provides: {"os_string_into_bstring"}
// Dependencies: {}
# [doc = " Like [`into_bstr()`], but takes `OsString` as input for a lossless, but fallible, conversion."] pub fn os_string_into_bstring (path : OsString) -> Result < BString , Utf8Error > { let path = try_into_bstr (Cow :: Owned (path . into ())) ? ; match path { Cow :: Borrowed (_path) => unreachable ! ("borrowed cows stay borrowed") , Cow :: Owned (path) => Ok (path) , } }
};
}
