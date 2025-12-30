// Generated macro for OpenDir (struct)
macro_rules! Depcrate_shims_unix_fsOpenDir {
() => {
// Module: crate::shims::unix::fs
// Provides: {"OpenDir"}
// Dependencies: {}
# [doc = " An open directory, tracked by DirHandler."] # [derive (Debug)] struct OpenDir { # [doc = " The directory reader on the host."] read_dir : ReadDir , # [doc = " The most recent entry returned by readdir()."] # [doc = " Will be freed by the next call."] entry : Option < Pointer > , }
};
}
