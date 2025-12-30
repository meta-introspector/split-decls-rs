// Generated macro for DotsNext (enum)
macro_rules! Depcrate_fs_dirDotsNext {
() => {
// Module: crate::fs::dir
// Provides: {"DotsNext"}
// Dependencies: {}
# [doc = " The dot directories that need to be listed before actual files, if any."] # [doc = " If these aren’t being printed, then `FilesNext` is used to skip them."] enum DotsNext { # [doc = " List the `.` directory next."] Dot , # [doc = " List the `..` directory next."] DotDot , # [doc = " Forget about the dot directories and just list files."] Files , }
};
}
