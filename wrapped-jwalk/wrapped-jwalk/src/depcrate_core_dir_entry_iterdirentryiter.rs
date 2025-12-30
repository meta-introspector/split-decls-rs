// Generated macro for DirEntryIter (struct)
macro_rules! Depcrate_core_dir_entry_iterDirEntryIter {
() => {
// Module: crate::core::dir_entry_iter
// Provides: {"DirEntryIter"}
// Dependencies: {}
# [doc = " DirEntry iterator from `WalkDir.into_iter()`."] # [doc = ""] # [doc = " Yields entries from recursive traversal of filesystem."] pub struct DirEntryIter < C : ClientState > { min_depth : usize , pub (crate) read_dir_iter : Option < Peekable < ReadDirIter < C > > > , read_dir_results_stack : Vec < vec :: IntoIter < Result < DirEntry < C > > > > , }
};
}
