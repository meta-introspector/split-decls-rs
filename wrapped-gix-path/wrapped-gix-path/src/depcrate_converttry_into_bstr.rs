// Generated macro for try_into_bstr (function)
macro_rules! Depcrate_converttry_into_bstr {
() => {
// Module: crate::convert
// Provides: {"try_into_bstr"}
// Dependencies: {}
# [doc = " Convert the given path either into its raw bytes on Unix or its UTF-8 encoded counterpart on Windows."] # [doc = ""] # [doc = " On Windows, if the source `Path`` contains ill-formed, lone surrogates, the UTF-8 conversion will fail"] # [doc = " causing `Utf8Error` to be returned."] pub fn try_into_bstr < 'a > (path : impl Into < Cow < 'a , Path > >) -> Result < Cow < 'a , BStr > , Utf8Error > { let path = path . into () ; let path_str = match path { Cow :: Owned (path) => Cow :: Owned ({ # [cfg (unix)] let p : BString = { use std :: os :: unix :: ffi :: OsStringExt ; path . into_os_string () . into_vec () . into () } ; # [cfg (target_os = "wasi")] let p : BString = { use std :: os :: wasi :: ffi :: OsStringExt ; path . into_os_string () . into_vec () . into () } ; # [cfg (not (any (unix , target_os = "wasi")))] let p : BString = path . into_os_string () . into_string () . map_err (| _ | Utf8Error) ? . into () ; p }) , Cow :: Borrowed (path) => Cow :: Borrowed ({ # [cfg (unix)] let p : & BStr = { use std :: os :: unix :: ffi :: OsStrExt ; path . as_os_str () . as_bytes () . into () } ; # [cfg (target_os = "wasi")] let p : & BStr = { use std :: os :: wasi :: ffi :: OsStrExt ; path . as_os_str () . as_bytes () . into () } ; # [cfg (not (any (unix , target_os = "wasi")))] let p : & BStr = path . to_str () . ok_or (Utf8Error) ? . as_bytes () . into () ; p }) , } ; Ok (path_str) }
};
}
