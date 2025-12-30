// Generated macro for tarball_variant (function)
macro_rules! Depcrate_manifesttarball_variant {
() => {
// Module: crate::manifest
// Provides: {"tarball_variant"}
// Dependencies: {}
fn tarball_variant (builder : & mut Builder , base : & Path , ext : & str) -> Option < PathBuf > { let mut path = base . to_path_buf () ; path . set_extension (ext) ; record_shipped_file (builder , path) }
};
}
