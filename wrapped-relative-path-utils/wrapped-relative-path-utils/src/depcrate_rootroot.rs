// Generated macro for Root (struct)
macro_rules! Depcrate_rootRoot {
() => {
// Module: crate::root
// Provides: {"Root"}
// Dependencies: {}
# [doc = " An open root directory from which relative paths can be opened."] # [doc = ""] # [doc = " In contrast to using APIs such as [`RelativePath::to_path`], this does not"] # [doc = " require allocations to open a path."] # [doc = ""] # [doc = " This is achieved by keeping an open handle to the directory and using"] # [doc = " platform-specific APIs to open a relative path, such as [`openat`] on `unix`."] # [doc = ""] # [doc = " [`openat`]: https://linux.die.net/man/2/openat"] pub struct Root { inner : imp :: Root , }
};
}
