// Generated macro for impl_109 (impl)
macro_rules! Depcrate_tokio_open_optionsimpl_109 {
() => {
// Module: crate::tokio::open_options
// Provides: {"impl_109"}
// Dependencies: {}
# [cfg (unix)] impl OpenOptions { # [doc = " Sets the mode bits that a new file will be created with."] # [doc = ""] # [doc = " Wrapper for [`tokio::fs::OpenOptions::mode`]."] pub fn mode (& mut self , mode : u32) -> & mut OpenOptions { self . tokio . mode (mode) ; self } # [doc = " Passes custom flags to the `flags` argument of `open`."] # [doc = ""] # [doc = " Wrapper for [`tokio::fs::OpenOptions::custom_flags`]."] pub fn custom_flags (& mut self , flags : i32) -> & mut OpenOptions { self . tokio . custom_flags (flags) ; self } }
};
}
