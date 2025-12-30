// Generated macro for impl_32 (impl)
macro_rules! Depcrateimpl_32 {
() => {
// Module: crate
// Provides: {"impl_32"}
// Dependencies: {}
# [doc = " *Requires Rust 1.68 or newer.*"] # [cfg (path_buf_deref_mut)] # [allow (clippy :: incompatible_msrv)] impl std :: ops :: DerefMut for Utf8PathBuf { fn deref_mut (& mut self) -> & mut Self :: Target { unsafe { Utf8Path :: assume_utf8_mut (& mut self . 0) } } }
};
}
