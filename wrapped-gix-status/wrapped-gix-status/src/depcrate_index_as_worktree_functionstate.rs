// Generated macro for State (struct)
macro_rules! Depcrate_index_as_worktree_functionState {
() => {
// Module: crate::index_as_worktree::function
// Provides: {"State"}
// Dependencies: {}
struct State < 'a , 'b > { buf : Vec < u8 > , buf2 : Vec < u8 > , timestamp : FileTime , # [doc = " This is the cheap stack that only assure that we don't go through symlinks."] # [doc = " It's always used to get the path to perform an lstat on."] path_stack : SymlinkCheck , # [doc = " This is the expensive stack that will need to check for `.gitattributes` files each time"] # [doc = " it changes directory. It's only used when we know we have to read a worktree file, which in turn"] # [doc = " requires attributes to drive the filter configuration."] attr_stack : gix_worktree :: Stack , filter : gix_filter :: Pipeline , path_backing : & 'b gix_index :: PathStorageRef , options : & 'a Options , skipped_by_pathspec : & 'a AtomicUsize , skipped_by_entry_flags : & 'a AtomicUsize , symlink_metadata_calls : & 'a AtomicUsize , entries_to_update : & 'a AtomicUsize , racy_clean : & 'a AtomicUsize , worktree_bytes : & 'a AtomicU64 , worktree_reads : & 'a AtomicUsize , odb_bytes : & 'a AtomicU64 , odb_reads : & 'a AtomicUsize , }
};
}
