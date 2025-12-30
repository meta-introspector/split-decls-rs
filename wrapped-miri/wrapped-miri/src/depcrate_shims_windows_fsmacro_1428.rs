// Generated macro for macro_1428 (macro)
macro_rules! Depcrate_shims_windows_fsmacro_1428 {
() => {
// Module: crate::shims::windows::fs
// Provides: {"macro_1428"}
// Dependencies: {}
bitflags ! { # [derive (PartialEq)] struct FileAttributes : u32 { const ZERO = 0 ; const NORMAL = 1 << 0 ; # [doc = " This must be passed to allow getting directory handles. If not passed, we error on trying"] # [doc = " to open directories"] const BACKUP_SEMANTICS = 1 << 1 ; # [doc = " Open a reparse point as a regular file - this is basically similar to 'readlink' in Unix"] # [doc = " terminology. A reparse point is a file with custom logic when navigated to, of which"] # [doc = " a symlink is one specific example."] const OPEN_REPARSE = 1 << 2 ; } }
};
}
