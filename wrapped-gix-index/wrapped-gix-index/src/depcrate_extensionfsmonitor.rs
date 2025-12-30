// Generated macro for FsMonitor (struct)
macro_rules! Depcrate_extensionFsMonitor {
() => {
// Module: crate::extension
// Provides: {"FsMonitor"}
// Dependencies: {}
# [doc = " The extension for keeping state on recent information provided by the filesystem monitor."] # [allow (dead_code)] # [derive (Clone)] pub struct FsMonitor { token : fs_monitor :: Token , # [doc = " if a bit is true, the respective entry is NOT valid as per the fs monitor."] entry_dirty : gix_bitmap :: ewah :: Vec , }
};
}
