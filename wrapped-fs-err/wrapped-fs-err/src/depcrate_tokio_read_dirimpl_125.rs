// Generated macro for impl_125 (impl)
macro_rules! Depcrate_tokio_read_dirimpl_125 {
() => {
// Module: crate::tokio::read_dir
// Provides: {"impl_125"}
// Dependencies: {}
# [cfg (unix)] impl DirEntry { # [doc = " Returns the underlying `d_ino` field in the contained `dirent` structure."] # [doc = ""] # [doc = " Wrapper around [`tokio::fs::DirEntry::ino`]."] pub fn ino (& self) -> u64 { self . tokio . ino () } }
};
}
