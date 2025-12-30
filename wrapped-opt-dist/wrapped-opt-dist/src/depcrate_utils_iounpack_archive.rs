// Generated macro for unpack_archive (function)
macro_rules! Depcrate_utils_iounpack_archive {
() => {
// Module: crate::utils::io
// Provides: {"unpack_archive"}
// Dependencies: {}
pub fn unpack_archive (path : & Utf8Path , dest_dir : & Utf8Path) -> anyhow :: Result < () > { log :: info ! ("Unpacking directory `{path}` into `{dest_dir}`") ; assert ! (path . as_str () . ends_with (".tar.xz")) ; let file = File :: open (path . as_std_path ()) ? ; let file = xz :: read :: XzDecoder :: new (file) ; let mut archive = tar :: Archive :: new (file) ; archive . unpack (dest_dir . as_std_path ()) ? ; Ok (()) }
};
}
