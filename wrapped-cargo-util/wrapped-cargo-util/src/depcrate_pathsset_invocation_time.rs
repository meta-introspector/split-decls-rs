// Generated macro for set_invocation_time (function)
macro_rules! Depcrate_pathsset_invocation_time {
() => {
// Module: crate::paths
// Provides: {"set_invocation_time"}
// Dependencies: {}
# [doc = " Record the current time on the filesystem (using the filesystem's clock)"] # [doc = " using a file at the given directory. Returns the current time."] pub fn set_invocation_time (path : & Path) -> Result < FileTime > { let timestamp = path . join ("invoked.timestamp") ; write (& timestamp , "This file has an mtime of when this was started." ,) ? ; let ft = mtime (& timestamp) ? ; tracing :: debug ! ("invocation time for {:?} is {}" , path , ft) ; Ok (ft) }
};
}
