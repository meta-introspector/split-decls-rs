// Generated macro for copy_dir_all (function)
macro_rules! Depcrate_fscopy_dir_all {
() => {
// Module: crate::fs
// Provides: {"copy_dir_all"}
// Dependencies: {}
# [doc = " Copy a directory into another. This will not traverse symlinks; instead, it will create new"] # [doc = " symlinks pointing at target paths that symlinks in the original directory points to."] pub fn copy_dir_all (src : impl AsRef < Path > , dst : impl AsRef < Path >) { fn copy_dir_all_inner (src : impl AsRef < Path > , dst : impl AsRef < Path >) -> io :: Result < () > { let dst = dst . as_ref () ; if ! dst . is_dir () { std :: fs :: create_dir_all (& dst) ? ; } for entry in std :: fs :: read_dir (src) ? { let entry = entry ? ; let ty = entry . file_type () ? ; if ty . is_dir () { copy_dir_all_inner (entry . path () , dst . join (entry . file_name ())) ? ; } else if ty . is_symlink () { copy_symlink_raw (ty , entry . path () , dst . join (entry . file_name ())) ? ; } else { std :: fs :: copy (entry . path () , dst . join (entry . file_name ())) ? ; } } Ok (()) } if let Err (e) = copy_dir_all_inner (& src , & dst) { panic ! ("failed to copy `{}` to `{}`: {:?}" , src . as_ref () . display () , dst . as_ref () . display () , e) ; } }
};
}
