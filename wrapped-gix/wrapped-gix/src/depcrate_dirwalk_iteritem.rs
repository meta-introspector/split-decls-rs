// Generated macro for Item (struct)
macro_rules! Depcrate_dirwalk_iterItem {
() => {
// Module: crate::dirwalk::iter
// Provides: {"Item"}
// Dependencies: {}
# [doc = " An entry of the directory walk as returned by the [iterator](Iter)."] pub struct Item { # [doc = " The directory entry."] pub entry : gix_dir :: Entry , # [doc = " `collapsed_directory_status` is `Some(dir_status)` if this entry was part of a directory with the given"] # [doc = " `dir_status` that wasn't the same as the one of `entry` and if [gix_dir::walk::Options::emit_collapsed] was"] # [doc = " [gix_dir::walk::CollapsedEntriesEmissionMode::OnStatusMismatch]. It will also be `Some(dir_status)` if that option"] # [doc = " was [gix_dir::walk::CollapsedEntriesEmissionMode::All]."] pub collapsed_directory_status : Option < gix_dir :: entry :: Status > , }
};
}
