// Generated macro for walk_dir_no_dot_or_target (function)
macro_rules! Depcrate_utilswalk_dir_no_dot_or_target {
() => {
// Module: crate::utils
// Provides: {"walk_dir_no_dot_or_target"}
// Dependencies: {}
# [doc = " Walks all items excluding top-level dot files/directories and any target directories."] pub fn walk_dir_no_dot_or_target (p : impl AsRef < Path >) -> impl Iterator < Item = :: walkdir :: Result < :: walkdir :: DirEntry > > { WalkDir :: new (p) . into_iter () . filter_entry (| e | { e . path () . file_name () . is_none_or (| x | x != "target" && x . as_encoded_bytes () . first () . copied () != Some (b'.')) }) }
};
}
