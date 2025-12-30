// Generated macro for impl_28 (impl)
macro_rules! Depcrate_implsimpl_28 {
() => {
// Module: crate::impls
// Provides: {"impl_28"}
// Dependencies: {}
impl TryFrom < & std :: ffi :: OsStr > for Url { type Error = parse :: Error ; fn try_from (value : & std :: ffi :: OsStr) -> Result < Self , Self :: Error > { gix_path :: os_str_into_bstr (value) . expect ("no illformed UTF-8 on Windows") . try_into () } }
};
}
