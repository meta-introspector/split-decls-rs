// Generated macro for impl_13 (impl)
macro_rules! Depcrate_as_filenameimpl_13 {
() => {
// Module: crate::as_filename
// Provides: {"impl_13"}
// Dependencies: {}
impl Sealed for String { # [cfg (windows)] fn windows_filename < R > (self , function : impl FnOnce (* const u16) -> Result < R , Error > ,) -> Result < R , Error > { self . as_str () . windows_filename (function) } # [cfg (unix)] fn posix_filename < R > (mut self , function : impl FnOnce (* const core :: ffi :: c_char) -> Result < R , Error > ,) -> Result < R , Error > { if crate :: util :: check_null_bytes (self . as_bytes ()) ? { function (self . as_ptr () . cast ()) } else { self . push ('\0') ; function (self . as_ptr () . cast ()) } } }
};
}
