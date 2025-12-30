// Generated macro for Files (struct)
macro_rules! Depcrate_fs_dirFiles {
() => {
// Module: crate::fs::dir
// Provides: {"Files"}
// Dependencies: {}
# [doc = " Iterator over reading the contents of a directory as `File` objects."] # [allow (clippy :: struct_excessive_bools)] pub struct Files < 'dir , 'ig > { # [doc = " The internal iterator over the paths that have been read already."] inner : SliceIter < 'dir , DirEntry > , # [doc = " The directory that begat those paths."] dir : & 'dir Dir , # [doc = " Whether to include dotfiles in the list."] dotfiles : bool , # [doc = " Whether the `.` or `..` directories should be produced first, before"] # [doc = " any files have been listed."] dots : DotsNext , git : Option < & 'ig GitCache > , git_ignoring : bool , # [doc = " Whether symbolic links should be dereferenced when querying information."] deref_links : bool , # [doc = " Whether to calculate the directory size recursively"] total_size : bool , }
};
}
