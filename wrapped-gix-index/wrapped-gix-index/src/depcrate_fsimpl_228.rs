// Generated macro for impl_228 (impl)
macro_rules! Depcrate_fsimpl_228 {
() => {
// Module: crate::fs
// Provides: {"impl_228"}
// Dependencies: {}
# [doc = " Lifecycle"] impl Metadata { # [doc = " Obtain the metadata at `path` without following symlinks."] pub fn from_path_no_follow (path : & Path) -> Result < Self , std :: io :: Error > { # [cfg (not (windows))] { rustix :: fs :: lstat (path) . map (Metadata) . map_err (Into :: into) } # [cfg (windows)] path . symlink_metadata () . map (Metadata) } # [doc = " Obtain the metadata at `path` without following symlinks."] pub fn from_file (file : & std :: fs :: File) -> Result < Self , std :: io :: Error > { # [cfg (not (windows))] { rustix :: fs :: fstat (file) . map (Metadata) . map_err (Into :: into) } # [cfg (windows)] file . metadata () . map (Metadata) } }
};
}
