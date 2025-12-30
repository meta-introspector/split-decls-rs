// Generated macro for macro_541 (macro)
macro_rules! Depcrate_ffimacro_541 {
() => {
// Module: crate::ffi
// Provides: {"macro_541"}
// Dependencies: {}
ffi_fn ! { # [doc = " Returns a static ASCII (null terminated) string of the hyper version."] fn hyper_version () -> * const std :: ffi :: c_char { VERSION_CSTR . as_ptr () as _ } ?= std :: ptr :: null () }
};
}
