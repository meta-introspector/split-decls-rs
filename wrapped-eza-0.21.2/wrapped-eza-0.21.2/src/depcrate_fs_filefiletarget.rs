// Generated macro for FileTarget (enum)
macro_rules! Depcrate_fs_fileFileTarget {
() => {
// Module: crate::fs::file
// Provides: {"FileTarget"}
// Dependencies: {}
# [doc = " The result of following a symlink."] pub enum FileTarget < 'dir > { # [doc = " The symlink pointed at a file that exists."] Ok (Box < File < 'dir > >) , # [doc = " The symlink pointed at a file that does not exist. Holds the path"] # [doc = " where the file would be, if it existed."] Broken (PathBuf) , # [doc = " There was an IO error when following the link. This can happen if the"] # [doc = " file isn’t a link to begin with, but also if, say, we don’t have"] # [doc = " permission to follow it."] Err (io :: Error) , }
};
}
