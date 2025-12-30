// Generated macro for copy_symlink_raw (function)
macro_rules! Depcrate_fscopy_symlink_raw {
() => {
// Module: crate::fs
// Provides: {"copy_symlink_raw"}
// Dependencies: {}
fn copy_symlink_raw (ty : FileType , src : impl AsRef < Path > , dst : impl AsRef < Path >) -> io :: Result < () > { let target_path = std :: fs :: read_link (src) ? ; let new_symlink_path = dst . as_ref () ; # [cfg (windows)] { use std :: os :: windows :: fs :: FileTypeExt ; if ty . is_symlink_dir () { std :: os :: windows :: fs :: symlink_dir (& target_path , new_symlink_path) ? ; } else { std :: os :: windows :: fs :: symlink_file (& target_path , new_symlink_path) ? ; } } # [cfg (unix)] { let _ = ty ; std :: os :: unix :: fs :: symlink (target_path , new_symlink_path) ? ; } # [cfg (not (any (windows , unix)))] { let _ = ty ; unimplemented ! ("unsupported target") ; } Ok (()) }
};
}
