// Generated macro for Sealed (trait)
macro_rules! Depcrate_as_filenameSealed {
() => {
// Module: crate::as_filename
// Provides: {"Sealed"}
// Dependencies: {}
pub (crate) trait Sealed { # [cfg (windows)] # [doc (hidden)] fn windows_filename < R > (self , function : impl FnOnce (* const u16) -> Result < R , crate :: Error > ,) -> Result < R , crate :: Error > ; # [cfg (unix)] # [doc (hidden)] fn posix_filename < R > (self , function : impl FnOnce (* const core :: ffi :: c_char) -> Result < R , crate :: Error > ,) -> Result < R , crate :: Error > ; }
};
}
