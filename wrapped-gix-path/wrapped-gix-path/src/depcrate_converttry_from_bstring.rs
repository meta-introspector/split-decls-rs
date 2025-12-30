// Generated macro for try_from_bstring (function)
macro_rules! Depcrate_converttry_from_bstring {
() => {
// Module: crate::convert
// Provides: {"try_from_bstring"}
// Dependencies: {}
# [doc = " Similar to [`try_from_bstr()`], but takes and produces owned data."] pub fn try_from_bstring (input : impl Into < BString >) -> Result < PathBuf , Utf8Error > { let input = input . into () ; # [cfg (unix)] let p = { use std :: os :: unix :: ffi :: OsStringExt ; std :: ffi :: OsString :: from_vec (input . into ()) . into () } ; # [cfg (target_os = "wasi")] let p : PathBuf = { use std :: os :: wasi :: ffi :: OsStringExt ; std :: ffi :: OsString :: from_vec (input . into ()) . into () } ; # [cfg (not (any (unix , target_os = "wasi")))] let p = { use bstr :: ByteVec ; PathBuf :: from ({ let v : Vec < _ > = input . into () ; v } . into_string () . map_err (| _ | Utf8Error) ? ,) } ; Ok (p) }
};
}
