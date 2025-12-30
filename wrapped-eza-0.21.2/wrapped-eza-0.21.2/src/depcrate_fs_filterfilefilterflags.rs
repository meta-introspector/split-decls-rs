// Generated macro for FileFilterFlags (enum)
macro_rules! Depcrate_fs_filterFileFilterFlags {
() => {
// Module: crate::fs::filter
// Provides: {"FileFilterFlags"}
// Dependencies: {}
# [doc = " Flags used to manage the **file filter** process"] # [derive (PartialEq , Eq , Debug , Clone)] pub enum FileFilterFlags { # [doc = " Whether to reverse the sorting order. This would sort the largest"] # [doc = " files first, or files starting with Z, or the most-recently-changed"] # [doc = " ones, depending on the sort field."] Reverse , # [doc = " Whether to only show directories."] OnlyDirs , # [doc = " Whether to only show files."] OnlyFiles , # [doc = " Whether to ignore symlinks"] NoSymlinks , # [doc = " Whether to explicitly show symlinks"] ShowSymlinks , # [doc = " Whether directories should be listed first, and other types of file"] # [doc = " second. Some users prefer it like this."] ListDirsFirst , # [doc = " Whether directories should be listed as the last items, after other"] # [doc = " types of file. Some users prefer it like this."] ListDirsLast , }
};
}
