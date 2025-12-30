// Generated macro for impl_11 (impl)
macro_rules! Depcrate_as_filenameimpl_11 {
() => {
// Module: crate::as_filename
// Provides: {"impl_11"}
// Dependencies: {}
impl Sealed for & String { # [cfg (windows)] fn windows_filename < R > (self , function : impl FnOnce (* const u16) -> Result < R , Error > ,) -> Result < R , Error > { self . as_str () . windows_filename (function) } # [cfg (unix)] fn posix_filename < R > (self , function : impl FnOnce (* const core :: ffi :: c_char) -> Result < R , Error > ,) -> Result < R , Error > { self . as_str () . posix_filename (function) } }
};
}
